use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use httparse;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut headers = [httparse::EMPTY_HEADER; 16];

    // lê a requisição do cliente
    let bytes_read = stream.read(&mut buffer).unwrap();
    let mut request = httparse::Request::new(&mut headers);
    let _ = request.parse(&buffer[..bytes_read]);

    // imprime a requisição recebida
    println!("Request: {:?}", request);

    // envia uma resposta para o cliente
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, client!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    // aceita conexões e processa-as em threads separadas
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(_) => {
                println!("Connection failed!");
            }
        }
    }
}
