

use std::net::{TcpListener, TcpStream, SocketAddr};
use std::collections::HashMap;
use std::io::Read;



const SERVER_PORT: i32 = 9000;



fn main() {




    //Initilize Server - loading configs
    let listener = TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT))
        .expect("failed to bind to port");
    let mut connections: HashMap<UUID, Connection> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                connection_handler(stream, &mut connections);
            },
                Err(e) => {
                println!("Error incoming connection");
            }
        }
    }

}

struct Connection{
    UUID: String,
    timestamp: i32,
}

fn connection_handler(mut stream: TcpStream, _connections: &mut HashMap<UUID, Connection>) {

    let mut buffer = [0; 4096];
    match stream.read(&mut buffer) {

        Ok(bytes_read) => {

            let input = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("we got {}", input);  
        }
        Err(e) => {
            println!("Error handling buffer");
        }
    }
}

struct UUID {
    ID: String,
    Connection_Socket: SocketAddr,
}
