use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

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

        let data = std::str::from_utf8(&buf[..bytes_read]).expect("error getting data from buffer");

        println!("data: {:?}", data);

        if let Some((command, args)) = parse_data(data) {
            println!("command: {}", command);

            println!("args: {:?}", args);

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

fn parse_data(data: &str) -> Option<(String, Vec<&str>)> {
    println!("data: {}", data);

    let parts: Vec<&str> = data.trim_end_matches("\r\n").split("\r\n").collect();

    if parts.is_empty() {
        return None;
    }

    println!("parts: {:?}", parts);

    let command = parts[2];

    let mut args: Vec<&str> = parts.iter().skip(2).step_by(2).copied().collect();

    if command != "ping" && parts.len() == 3 {
        return None;
    };

    args.retain_mut(|&mut s| s != command);

    println!("command: {}", command);

    println!("args: {:?}", args);

    Some((command.to_lowercase(), args))
}
