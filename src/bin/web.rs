use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

enum ResponseType {
    StringResponse(String),
    NotFound,
}

use ResponseType::*;

fn main() {
    let address = "127.0.0.1:7878";
    println!("starting server at http://{address}");
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(s)) => s,
        _ => return,
    };
    println!("{request_line}");
    
    let response = match request_line.split(' ').collect::<Vec<_>>()[..] {
        ["GET", s, "HTTP/1.1"] => {
            match s.split('/').collect::<Vec<_>>()[1..] {
                [""] => StringResponse(fs::read_to_string("log_in.html").unwrap()),
                _ => NotFound,
            }
        },
        _ => NotFound,
    };

    handle_response(response, stream);
}

fn handle_response(response: ResponseType, mut stream: TcpStream) {
    let status_line = match response {
        StringResponse(_) => "HTTP/1.1 200 OK",
        NotFound => "HTTP/1.1 404 NOT FOUND"
    };
    
    let body = match response {
        StringResponse(contents) => {
            let length = contents.len();
            format!("Content-Length: {length}\r\n\r\n{contents}")
        },
        NotFound => {
            let contents = fs::read_to_string("404.html").unwrap();
            let length = contents.len();
            format!("Content-Length: {length}\r\n\r\n{contents}")
        }
    };

    let response = format!("{status_line}\r\n{body}");
    stream.write_all(response.as_bytes()).unwrap();
}