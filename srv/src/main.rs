use std::collections::HashMap;

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use tokio;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
//mod chat_channel;
const SERVER_PORT: i32 = 9000;

struct Channel {
    UUID: u32,
    userz: Vec<u32>,
    messages: Vec<String>,
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    //Initilize Server - loading configs
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT)).expect("failed to bind to port");
    let ch = Channel {
        UUID: 0,
        userz: vec![0, 1],
        messages: vec!["I like cookiez".to_string()],
    };
    let channelz = Arc::new(Mutex::new(ch));
    let channelz_clone = Arc::clone(&channelz);
    //maybe sockets?

    //    let rt = Runtime::new().unwrap();
    loop {
        let stream_accept = listener.accept();
        match stream_accept {
            Ok(stream) => {
                let stream_clone = &stream;
                tokio::spawn(async move {
                    sender_fn(stream_clone, channelz_clone).await;
                });
                let channelz_clone = Arc::clone(&channelz);
                tokio::spawn(async move {
                    receiver_fn(listener, channelz_clone).await;
                });
            }
            Err(a) => {
                println!("error stream");
            }
        }
    }
}
async fn receiver_fn(listener: TcpListener, channelz: Arc<Mutex<Channel>>) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new connection {}", stream.peer_addr().unwrap());
                connection_handler(stream) //, &mut connections); //, chat);
            }
            Err(e) => {
                println!("Error incoming connection: {}", e);
            }
        }
    }
}

async fn sender_fn(sender: TcpListener, channelz: Arc<Mutex<Channel>>) {
    loop {
        for stream in sender.incoming() {
            match stream {
                Ok(mut sender_stream) => {
                    let _ = sender_stream.write("I have cookiez".as_bytes());
                    println!("send msg");
                }
                Err(a) => {
                    println!("Error writing connection: {}", a);
                }
            }
        }
    }
}

struct Connection {
    //   UUID: String,
    timestamp: i32,
}

fn connection_handler(
    mut stream: TcpStream,
    //    _connections: &mut HashMap<UUID, Connection>,
    //    chat: Chat,
) {
    let mut buffer = [0; 4096];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let input = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("{}", input);
            let _ = stream.write(format!("msg: {}", input).as_bytes());
        }
        Err(e) => {
            println!("Error handling buffer: {}", e);
        }
    }
}
