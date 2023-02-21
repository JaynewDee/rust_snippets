mod server {
    use tokio::{
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        net::{tcp::ReadHalf, TcpListener},
        sync::broadcast,
    };

    pub struct Server;

    impl Default for Server {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Server {
        fn new() -> Server {
            Server
        }

        pub async fn listen(port: &str) {
            let listener = TcpListener::bind(format!("localhost:{port}"))
                .await
                .expect("Listener should have bound to port.");

            let (tx, _rx) = broadcast::channel(10);

            loop {
                let (mut socket, address) = listener.accept().await.unwrap();
                let tx = tx.clone();
                let mut rx = tx.subscribe();

                tokio::spawn(async move {
                    let (reader, mut writer) = socket.split();

                    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);

                    let mut line = String::new();
                    println!("{line}");
                    loop {
                        tokio::select! {

                            result = reader.read_line(&mut line) => {

                                if result.unwrap() == 0 {
                                    break;
                                }
                                tx.send((line.clone(), address)).unwrap();
                                line.clear();
                            }

                            result = rx.recv() => {

                                let (msg, other_addr) = result.unwrap();
                                if address != other_addr {
                                    writer.write_all(msg.as_bytes()).await.unwrap();
                                }

                            }
                        }
                    }
                });
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let port = "8080";
    server::Server::listen(port).await;
}
