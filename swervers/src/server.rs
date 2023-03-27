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
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };

    pub fn listen() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
            handle_connection(stream);
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        println!("{:?}", response);
        println!("Request: {:#?}", http_request);
        stream.write_all(response.as_bytes()).unwrap();
    }
}
