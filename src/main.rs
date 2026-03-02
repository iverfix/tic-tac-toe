pub mod board;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let mut board = board::Board::default();
    board.play(1, 1);
    board.print();
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn create_response(buffer: &[u8; 1024]) -> String {
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        fs::read_to_string("index.html").unwrap()
    } else {
        fs::read_to_string("404.html").unwrap()
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("Request {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    let content = create_response(&buffer);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        content.len(),
        content
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
