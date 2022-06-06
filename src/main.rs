use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::env;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();

  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  let get = b"GET / HTTP/1.1\r\n";

  let file_content = fs::read_to_string("index.html").unwrap().to_string();
  let contents = fs::read_to_string("index.html").unwrap();

  let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
                         "HTTP/1.1 200 OK",
                         contents.len(),
                         contents);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
