pub mod chat_server {
    use tokio::{
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        net::TcpListener,
        sync::broadcast,
    };

    pub async fn launch() {
        let listener = TcpListener::bind("localhost:8080")
            .await
            .expect("Future should have properly resolved with bind.");

        // Init broadcast queue with maximum capacity
        let (sndr, _rcvr) = broadcast::channel(10);

        // For every incoming request,
        loop {
            let (mut socket, addr) = listener
                .accept()
                .await
                .expect("Listener should have accepted socket binding request from client.");

            let sndr = sndr.clone();
            let mut rcvr = sndr.subscribe();

            tokio::spawn(async move {
                let (reader, mut writer) = socket.split();

                let mut reader = BufReader::new(reader);

                let mut line = String::new();

                loop {
                    tokio::select! {
                        result = reader.read_line(&mut line) => {
                            if result.unwrap() == 0 {
                                break;
                            }

                            sndr.send((line.clone(), addr)).unwrap();
                            line.clear()
                        }
                        result = rcvr.recv() => {
                            let (msg, other_addr) = result.unwrap();

                            if addr != other_addr {
                                writer.write_all(msg.as_bytes()).await.unwrap();
                            }
                        }
                    }
                }
            });
        }
    }
}

pub mod http_server {

    use std::{
        env::current_dir,
        fs::read_to_string,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
        path::PathBuf,
        sync::{mpsc, Arc, Mutex},
        thread::{self, JoinHandle},
    };

    struct Worker {
        id: usize,
        handle: JoinHandle<()>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            Worker {
                id,
                handle: thread::spawn(move || loop {
                    // Call to recv blocks so thread waits for a job from sender
                    let job = receiver.lock().unwrap().recv().unwrap();

                    println!("Worker {id} got a job; executing.");

                    job();
                }),
            }
        }
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for i in 0..size {
                workers.push(Worker::new(i, Arc::clone(&receiver)));
            }

            ThreadPool { workers, sender }
        }

        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.send(job).unwrap();
        }
    }

    pub fn listen() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let this_dir = get_working_dir()
            .expect("Expected function to return PathBuf to current working directory.");

        let statics_path =
            get_statics_dir(&this_dir).expect("Expected function to return path to statics.");
        let contents = read_to_string(statics_path).unwrap();
        let length = contents.len();

        let status_ok = "HTTP/1.1 200 OK";

        let response = format!("{status_ok}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }

    fn get_working_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = current_dir()?;
        Ok(path)
    }

    fn get_statics_dir(dirname: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path_to_index = dirname.join("statics/index.html");
        Ok(path_to_index)
    }
}
