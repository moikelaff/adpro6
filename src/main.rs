use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://127.0.0.1:7878");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.is_empty() {
        return;
    }

    let request_line = &http_request[0];
    let parts: Vec<&str> = request_line.split(' ').collect();
    if parts.len() < 2 {
        return;
    }

    let path = parts[1];
    let (status_line, contents) = if path == "/" {
        ("HTTP/1.1 200 OK", fs::read_to_string("hello.html").unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("not_avail.html").unwrap())
    };

    let length = contents.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);
    stream.write_all(response.as_bytes()).unwrap();
}
