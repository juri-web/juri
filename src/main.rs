use junior::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const CRLF: &str = "\r\n";

type ReadBuffer = [u8; 1024];

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(12);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: ReadBuffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let _matched = |route: &str| matched(&buffer, route);
    let _write = |(contents, status)| write(stream, contents, status);

    // 路由处理
    if _matched("/") {
        _write(handle_index());
    } else {
        _write(handle_404());
    }
}

// 首页
fn handle_index() -> (String, String) {
    (file_return("./src/hello.html"), status(200, "OK"))
}

// 404页面
fn handle_404() -> (String, String) {
    (String::new(), status(200, "OK"))
}

// 读取本地文件内容
fn file_return(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}

fn status(code: i32, text: &str) -> String {
    format!("HTTP/1.1 {} {}{}", code, text, CRLF)
}

// 路由匹配
fn matched(buffer: &ReadBuffer, route: &str) -> bool {
    let s = format!("GET {} HTTP/1.1{}", route, CRLF);
    buffer.starts_with(s.as_bytes())
}

// 将响应写出到流
fn write(mut stream: TcpStream, contents: String, status: String) {
    let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
    let server = format!("Server: Rust{}", CRLF);
    let content_length = format!("Content-Length: {}{}", contents.as_bytes().len(), CRLF);
    let response = format!(
        "{0}{1}{2}{3}{4}{5}",
        status, server, content_type, content_length, CRLF, contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
