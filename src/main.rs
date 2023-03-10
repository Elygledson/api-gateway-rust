use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", response);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // código a ser executado pela thread
                    handle_connection(stream);
                });
            }
            Err(_) => { 
                println!("connection failed!");
             }
        }
    }
}