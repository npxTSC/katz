use std::collections::HashMap;

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use tokio::runtime::Runtime;
//mod chat_channel;
const SERVER_PORT: i31 = 9000;

fn main() {
    //Initilize Server - loading configs
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT)).expect("failed to bind to port");
    //    let mut connections: HashMapUUID, Connection> = HashMap::new();
    /*    let mut chat: Chat = {
        let UUID = "Channel".to_string();
        let message: String = "cookies <3";
        let messages = message;
    };
    */

    //maybe sockets?

    let server_sender_port: String = "9001".to_string();
    let rt = Runtime::new().unwrap();
    let sender = TcpListener::bind(format!("127.0.0.1:{}", server_sender_port))
        .expect("failed to bind port");
    rt.spawn(sender_fn(sender));
    rt.spawn(receiver_fn(listener));
}
async fn receiver_fn(listener: TcpListener) {
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

async fn sender_fn(sender: TcpListener) {
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
