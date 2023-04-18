// Socket concurrency with tokio
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
////////////////
// Vanilla Concurrent HTTP
////////////////
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

////////////////////
// Actix HTTP
pub mod swerver_framework {
    use super::framework_socket::WebSocket;
    use actix_files::NamedFile;
    use actix_web::{
        middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
    };
    use actix_web_actors::ws;

    async fn index() -> impl Responder {
        NamedFile::open_async("./statics/index.html").await.unwrap()
    }

    async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
        ws::start(WebSocket::new(), &req, stream)
    }

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        log::info!("starting HTTP server at http://localhost:8787");

        // Here we're creating the server and binding it to port 8080.
        HttpServer::new(|| {
            App::new()
                // "/" is the path that we want to serve the `index.html` file from.
                .service(web::resource("/").to(index))
                .wrap(middleware::Logger::default())
        })
        .workers(2)
        .bind(("127.0.0.1", 8787))?
        .run()
        .await
    }
}

/////////////////////
// Actix WebSockets
/////////////////////
pub mod framework_socket {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use std::time::{Duration, Instant};

    use actix::prelude::*;
    use actix_web::web::Bytes;
    use actix_web_actors::ws;

    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
    const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

    pub struct WebSocket {
        hb: Instant,
    }

    impl WebSocket {
        pub fn new() -> Self {
            Self { hb: Instant::now() }
        }

        // This function will run on an interval, every 5 seconds to check
        // that the connection is still alive. If it's been more than
        // 10 seconds since the last ping, we'll close the connection.
        fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
            ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
                if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                    ctx.stop();
                    return;
                }

                ctx.ping(b"");
            });
        }
    }

    impl Actor for WebSocket {
        type Context = ws::WebsocketContext<Self>;

        // Start the heartbeat process for this connection
        fn started(&mut self, ctx: &mut Self::Context) {
            self.heartbeat(ctx);
        }
    }

    // The `StreamHandler` trait is used to handle the messages that are sent over the socket.
    impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
        fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
            match msg {
                // Ping/Pong will be used to make sure the connection is still alive
                Ok(ws::Message::Ping(msg)) => {
                    self.hb = Instant::now();
                    ctx.pong(&msg);
                }
                Ok(ws::Message::Pong(_)) => {
                    self.hb = Instant::now();
                }
                // Text will echo any text received back to the client (for now)
                Ok(ws::Message::Text(text)) => ctx.text(text),
                // Close will close the socket
                Ok(ws::Message::Close(reason)) => {
                    ctx.close(reason);
                    ctx.stop();
                }
                _ => ctx.stop(),
            }
        }
    }
}
