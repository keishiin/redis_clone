use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::parser::parse_data_new;

mod parser;

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let in_mem_store: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (mut stream, socket) = listener.accept().await.unwrap();

        println!("accepted new connection on {}", socket.port());

        let mut mem_store_clone = Arc::clone(&in_mem_store);

        tokio::spawn(async move {
            handle_connection(&mut stream, &mut mem_store_clone).await;
        });
    }
}

async fn handle_connection(
    stream: &mut TcpStream,
    store: &mut Arc<Mutex<HashMap<String, String>>>,
) {
    let mut buf = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buf).await.unwrap();

        if bytes_read == 0 {
            println!("no more bytes, connection is closed");

            break;
        }

        if let Some(mut data) = parse_data_new(&buf) {
            let command = data[0];
            data.retain(|&x| x != command);
            let args = &data;

            let response = match command.to_lowercase().as_str() {
                "ping" => "+PONG\r\n".to_string(),

                "echo" => format!(
                    "${}\r\n{}\r\n",
                    args.first().unwrap().len(),
                    args.first().unwrap()
                ),

                "set" => {
                    store.lock().unwrap().insert(
                        args.first().unwrap().to_string().clone(),
                        args[1].to_string().clone(),
                    );

                    "+OK\r\n".to_string()
                }

                "get" => {
                    match store
                        .lock()
                        .unwrap()
                        .get(args.first().unwrap().to_string().as_str())
                    {
                        None => "$-1\r\n".to_string(),
                        Some(v) => {
                            format!("${}\r\n{}\r\n", v.len(), v)
                        }
                    }
                }

                _ => "-Error unknown command\r\n".to_string(),
            };

            stream.write_all(response.as_bytes()).await.unwrap();
        } else {
            stream
                .write_all(b"-invalid format for redis protocol\r\n")
                .await
                .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_handle_connection_ping() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let store = Arc::new(Mutex::new(HashMap::new()));
        let store_clone = store.clone();

        tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut store_clone = store_clone.clone();
                tokio::spawn(async move {
                    handle_connection(&mut socket, &mut store_clone).await;
                });
            }
        });

        let mut stream = TcpStream::connect(addr).await.unwrap();

        stream.write_all(b"+1\r\n$4\r\nping\r\n").await.unwrap();

        let mut buf = [0; 1024];
        let bytes_read = stream.read(&mut buf).await.unwrap();
        let response = std::str::from_utf8(&buf[..bytes_read]).unwrap();

        assert_eq!(response, "+PONG\r\n");
    }

    #[tokio::test]
    async fn test_handle_connection_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let store = Arc::new(Mutex::new(HashMap::new()));
        let store_clone = store.clone();

        tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut store_clone = store_clone.clone();
                tokio::spawn(async move {
                    handle_connection(&mut socket, &mut store_clone).await;
                });
            }
        });

        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream
            .write_all(b"*2\r\n$4\r\nECHO\r\n$5\r\nhello\r\n")
            .await
            .unwrap();

        let mut buf = [0; 1024];
        let bytes_read = stream.read(&mut buf).await.unwrap();
        let response = std::str::from_utf8(&buf[..bytes_read]).unwrap();

        assert_eq!(response, "$5\r\nhello\r\n");
    }

    #[tokio::test]
    async fn test_handle_connection_get() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let store = Arc::new(Mutex::new(HashMap::new()));
        let store_clone = store.clone();
        store_clone
            .lock()
            .unwrap()
            .insert("mykey".to_string(), "hello".to_string());

        tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut store_clone = store_clone.clone();
                tokio::spawn(async move {
                    handle_connection(&mut socket, &mut store_clone).await;
                });
            }
        });

        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream
            .write_all(b"$6\r\nGET\r\n$5\r\nmykey\r\n")
            .await
            .unwrap();

        let mut buf = [0; 1024];
        let bytes_read = stream.read(&mut buf).await.unwrap();
        let response = std::str::from_utf8(&buf[..bytes_read]).unwrap();

        assert_eq!(response, "$5\r\nhello\r\n");
    }

    #[tokio::test]
    async fn test_handle_connection_set() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let store = Arc::new(Mutex::new(HashMap::new()));
        let store_clone = store.clone();
        store_clone
            .lock()
            .unwrap()
            .insert("mykey".to_string(), "hello".to_string());

        tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut store_clone = store_clone.clone();
                tokio::spawn(async move {
                    handle_connection(&mut socket, &mut store_clone).await;
                });
            }
        });

        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream
            .write_all(b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n")
            .await
            .unwrap();

        let mut buf = [0; 1024];
        let bytes_read = stream.read(&mut buf).await.unwrap();
        let response = std::str::from_utf8(&buf[..bytes_read]).unwrap();

        let store_lock = store.lock().unwrap();

        assert_eq!(response, "+OK\r\n");
        assert_eq!(store_lock.get("key"), Some(&"value".to_string()));
    }
}
