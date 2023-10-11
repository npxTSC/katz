

use std::net::TcpStream;
use std::io::Write;
use std::io::{self};







fn main() {

    const SERVER_IP: &str = "127.0.0.1";
    const SERVER_PORT: u32 = 9000;


    match TcpStream::connect(format!("{}:{}",SERVER_IP, SERVER_PORT)) {
        Ok(mut stream) => {

            println!("Connection successfully established");
                

            let mut content: String = Default::default();
            

            loop {

                io::stdin().read_line(&mut content)
                    .expect("I couldnt read the message");

                stream.write(content.as_bytes())
                    .expect("error: message could not be delivered");

                content = "";
            }

        }
        Err(e) => {
            println!("Error Connection refused: {}", e);
        }
    }







}
