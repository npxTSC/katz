use std::collections::HashMap;

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::time::sleep;
//mod chat_channel;
const SERVER_PORT: i32 = 9000;

struct Channel {
    UUID: u32,
    userz: Vec<u32>,
    messages: Vec<String>,
}

#[tokio::main]
async fn main() {
    //Initilize Server - loading configs
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT)).expect("failed to bind to port");
    let ch = Channel {
        UUID: 0,
        userz: vec![0, 1],
        messages: vec!["I like cookiez".to_string()],
    };
    let channelz = Arc::new(Mutex::new(ch));
    //maybe sockets?

    loop {
        //    let rt = Runtime::new().unwrap();
        let stream_accept = listener.accept();
        match stream_accept {
            Ok(streamz) => {
                let stream = streamz.0;
                let stream_clone = stream.try_clone().expect("Error cloning stream");
                let channelz_clone = Arc::clone(&channelz);
                let channelz_clone_2 = Arc::clone(&channelz);
                tokio::spawn(async move {
                    sender_fn(stream_clone, channelz_clone).await;
                });

                tokio::spawn(async move {
                    receiver_fn(stream, channelz_clone_2).await;
                });
            }

            Err(a) => {
                println!("error stream {}", a);
            }
        }
        let _ = sleep(Duration::new(1, 0));
    }
}
async fn receiver_fn(mut stream: TcpStream, _channelz: Arc<Mutex<Channel>>) {
    println!("new connection {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 4096];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let input = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("{}", input);
            _channelz.lock().await.messages.push(input.to_string());
        }
        Err(e) => {
            println!("Error handling buffer: {}", e);
        }
    }
}

async fn sender_fn(mut stream: TcpStream, channelz: Arc<Mutex<Channel>>) {
    let messagez: &Vec<String> = &channelz.lock().await.messages;
    //let _ = stream.write("I have cookiez".as_bytes());
    for msg in messagez {
        let message = msg.as_bytes();
        let _ = stream.write_all(message);
    }

    println!("send msg");
}
struct Connection {
    //   UUID: String,
    timestamp: i32,
}
