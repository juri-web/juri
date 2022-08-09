use super::super::api;
use super::super::setting;
use super::thread::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const CRLF: &str = "\r\n";

type ReadBuffer = [u8; 1024];

pub fn run() {
    let listener = TcpListener::bind(setting::ADDRESS).unwrap();
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
        _write(api::views::handle_index());
    } else {
        _write(api::views::handle_404());
    }
}

pub fn status(code: i32, text: &str) -> String {
    format!("HTTP/1.1 {} {}{}", code, text, CRLF)
}

// 读取本地文件内容
pub fn template_file_return(file_name: &str) -> String {
    fs::read_to_string(setting::TEMPLATE_PATH.to_owned() + file_name).unwrap()
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
