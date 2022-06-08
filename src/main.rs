use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::env;
use std::error::Error;
use httparse;
use try_catch::*;
use std::io;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let buffer_string = String::from_utf8_lossy(&buffer[..]);

  //println!("Request: {}", buffer_string);

  let path = buffer_string.split_once("\n")
    .map(|o| o.0)
    .unwrap_or_else(|| &buffer_string)
    .replace("GET ", "")
    .replace("POST ", "")
    .replace(" HTTP/1.1", "");

  send_response(&mut stream, &path);
}

fn send_response(stream: &mut TcpStream, path: &str) {
  catch! {
    try {
      if path.ends_with("/") {
        println!("{}", format!("html{}index.html", path));

        let pathString = format!("html{}index.html", path);
        let contents = fs::read_to_string(pathString)?;

        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
                               "HTTP/1.1 200 OK",
                               contents.len(),
                               contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
      } else {
        println!("{}", format!("html{}", path));

        let pathString = format!("html{}", path);
        let contents = fs::read_to_string(pathString)?;

        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
                               "HTTP/1.1 200 OK",
                               contents.len(),
                               contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
      }
    }
    catch err {
      println!("{}\nError: {}", format!("html{}", path), err);

      let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
                             "HTTP/1.1 404 NOT FOUND",
                             0,
                             "404 NOT FOUND");

      stream.write(response.as_bytes()).unwrap();
      stream.flush().unwrap();
    }
  }
}