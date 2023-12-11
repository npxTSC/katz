use std::collections::HashMap;

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
//mod chat_channel;
const SERVER_PORT: i32 = 9000;

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
