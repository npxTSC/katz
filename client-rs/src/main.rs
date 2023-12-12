use std::io::{self};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
fn main() {
    const SERVER_IP: &str = "127.0.0.1";
    const SERVER_PORT: u32 = 9000;
    let SERVER_ADDRESS: String = format!("{}:{}", SERVER_IP, SERVER_PORT).to_string();
    let async_task = Runtime::new().unwrap();

    loop {
        match TcpStream::connect(SERVER_ADDRESS.clone()) {
            Ok(mut stream) => {
                println!("Connection successfully established");

                let mut content: String = Default::default();

                io::stdin()
                    .read_line(&mut content)
                    .expect("I couldnt read the message");

                stream
                    .write(content.as_bytes())
                    .expect("error: message could not be delivered");

                content = "".to_string();
                connection_handler(stream.try_clone().unwrap());
            }
            Err(e) => {
                println!("Error Connection refused: {}", e);
            }
        }
    }
}

//async fn listen(stream: TcpStream) {
//    let listener = TcpListener::bind("127.0.0.1:4000").expect("failed to bind incoming port");

//    for stream in listener.incoming() {
//    match stream {
//      Ok(stream) => {
//    connection_handler(stream);
//    }
//    Err(e) => {
//      println!("Error incoming connection: {}", e);
// }
//    }
//    }
//}

fn connection_handler(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let input = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("{}", input);
        }
        Err(e) => {
            println!("Error handling incoming buffer: {}", e);
        }
    }
}