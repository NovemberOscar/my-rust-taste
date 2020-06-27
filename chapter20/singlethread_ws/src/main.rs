use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let http_get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(http_get) {
        ("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\n",
         "/Users/k.seonghyeon/Documents/GitHub/my-rust-taste/chapter20/singlethread_ws/src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html; charset=UTF-8\r\n\n",
         "/Users/k.seonghyeon/Documents/GitHub/my-rust-taste/chapter20/singlethread_ws/src/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
