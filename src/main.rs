use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::env;
use std::thread;

fn main() {
  let listener_http  = TcpListener::bind("0.0.0.0:80").unwrap();
  let listener_https = TcpListener::bind("0.0.0.0:443").unwrap();

  thread::spawn(|| { handle_listener(listener_http); });
  thread::spawn(|| { handle_listener(listener_https); });
}

fn handle_listener(mut listener: TcpListener) {
  for stream in listener.incoming() {
    let stream = stream.unwrap();

    thread::spawn(|| {
      handle_connection(stream);
    });
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
    .replace("PUT ", "")
    .replace("HEAD ", "")
    .replace("TRACE ", "")
    .replace("DELETE ", "")
    .replace("CONNECT ", "")
    .replace("OPTIONS ", "")
    .replace(" HTTP/1.1", "")
    .replace("\r", "");

  if env::consts::OS == "windows" {
    let path = path.replace("/", "\\");

    send_response(&mut stream, path);
  } else {
    send_response(&mut stream, path);
  }
}

fn send_response(stream: &mut TcpStream, path: String) {
  if path.ends_with(&"/") || path.ends_with(&"\\") {
    let path_string = format!("html{}index.html", path);

    if fs::metadata(&path_string).is_ok() {
      let contents = fs::read_to_string(path_string).unwrap();

      let response = format!("{}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", "HTTP/1.1 200 OK", contents.len(), contents);

      stream.write(response.as_bytes()).unwrap();
      stream.flush().unwrap();
    } else {
      let contents = format!("<html><body><h1>Index of {}</h1><hr/></body></html>", path);

      let response = format!("{}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", "HTTP/1.1 200 OK", contents.len(), contents);

      stream.write(response.as_bytes()).unwrap();
      stream.flush().unwrap();
    }
  } else {
    let path_string = format!("html{}", path);
    if fs::metadata(&path_string).is_ok() {
      let contents = fs::read_to_string(path_string).unwrap();

      let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", "HTTP/1.1 200 OK", contents.len(), contents);

      stream.write(response.as_bytes()).unwrap();
      stream.flush().unwrap();
    } else {
      let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", "HTTP/1.1 404 Not Found", 0, "");

      stream.write(response.as_bytes()).unwrap();
      stream.flush().unwrap();
    }
  }
}