use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::env;
use std::thread;
use log::{info, warn};

fn main() {
  info!(target: "main_logs", "Rusty server is staring.");

  check_config(env::consts::OS);

  let listener_http  = TcpListener::bind("0.0.0.0:8080").unwrap();
  let listener_https = TcpListener::bind("0.0.0.0:8443").unwrap();

  thread::spawn(|| { handle_listener(listener_http); }).join().unwrap();
  thread::spawn(|| { handle_listener(listener_https); }).join().unwrap();
}

// config file check

fn check_config(os: &str) {
  if os == "linux" {
    if fs::metadata("/etc/rusty/main.conf").is_ok() {
      info!(target: "main_logs", "Config file found.");
    } else {
      warn!(target: "main_logs", "Config file not found.");
      info!(target: "main_logs", "Creating config file.");

      fs::create_dir_all("/etc/rusty").unwrap();
      fs::write("/etc/rusty/main.conf", "HttpPort 80\nHttpsPort 443\n\nServerRoot /var/www/public_html").unwrap();
    }
  }

  parse_config(os);
}

fn parse_config(os: &str) {
  if os == "linux" {
    let config_data = fs::read_to_string("/etc/rusty/main.conf").unwrap();

    let mut config_lines = config_data.lines();
  }
}

// connection handler

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