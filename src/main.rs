use std::fs;
use std::io::prelude::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {

    // 绑定到一个端口
    let listen = TcpListener::bind("127.0.0.1:8888").expect("TCP 绑定失败");
    for stream in listen.incoming() {
        let stream = stream.expect("TCP 链接失败");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("TCP 读取失败");

    let response : String = if is_get_method(&buffer) {
        let contents = read_file("resource/hello.html");
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        )
    } else {
        get_not_found_response()
    };
    stream.write(response.as_bytes()).expect("TCP 写入失败");
    stream.flush().expect("TCP 刷新失败");
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(&("文件".to_string() + path + "读取失败").to_string())
}

fn get_not_found_response() -> String {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n";
    let contents = read_file("resource/404.html");
    format!("{}Content-Length: {}\r\n\r\n{}", status_line, contents.len(), contents)
}

fn is_get_method(buffer: &[u8; 1024]) -> bool {
    let get = b"GET / HTTP/1.1\r\n";
    buffer.starts_with(get)
}
