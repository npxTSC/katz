#[allow(unused_imports)]
#[allow(unused_variables)]
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self};
use std::io::{stdout, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    const SERVER_IP: &str = "127.0.0.1";
    const SERVER_PORT: u32 = 9000;
    let server_address: String = format!("{}:{}", SERVER_IP, SERVER_PORT).to_string();

    println!("Connection successfully established");
    let server_address_copy = server_address.clone();
    tokio::spawn(async move {
        send_messagez(server_address.clone()).await;
    });
    //    tokio::spawn(async move {
    //        receiver_fn(server_address_copy).await;
    //    });
}
async fn send_messagez(server_address: String) {
    loop {
        let mut content: String = Default::default();

        io::stdin()
            .read_line(&mut content)
            .expect("I couldnt read the message");

        match TcpStream::connect(server_address.clone()) {
            Ok(mut stream) => {
                stream
                    .write(content.as_bytes())
                    .expect("error: message could not be delivered");

                //content = "".to_string();
                //                connection_handler(stream.try_clone().unwrap());
                receiver_fn(stream.try_clone().unwrap()).await;
            }
            Err(a) => {
                println!("error {}", a);
            }
        }
    }
}
struct ChatChannel {
    UUID: u32,
    messages: Vec<String>,
}
async fn receiver_fn(mut stream: TcpStream) {
    //    loop {
    //      match TcpStream::connect(server_address.clone()) {
    //        Ok(mut stream) => {
    clear_terminal().await;

    println!("new connection {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 4096];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let input = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("$~: {}", input);
        }
        Err(e) => {
            println!("Error handling buffer: {}", e);
        }
    }

    let _ = tokio::time::sleep(tokio::time::Duration::from_secs(4));
    //      }
    //    Err(a) => {
    //           println!("error {}", a);
    //       }
    //   }
    //}
}
async fn clear_terminal() {
    let mut stdout = stdout();
    let progress = execute!(stdout, Clear(ClearType::All));
    match progress {
        Ok(a) => {
            println!("terminal cleared");
            let pro = stdout.flush();
            match pro {
                Ok(a) => {
                    println!("flush completed");
                }
                Err(a) => {
                    println!("error flushing {}", a);
                }
            }
        }
        Err(a) => {
            println!("error: {}", a);
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
