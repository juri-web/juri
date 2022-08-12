use regex::Regex;

use super::router::{Router, handle_router};
use std::io::prelude::*;
use std::net::TcpStream;
const CRLF: &str = "\r\n";
/**1KB */
type ReadBuffer = [u8; 1024];

pub struct Context {
    stream: TcpStream,
    pub method: String,
    pub full_path: String,
    pub path: String,
    // params Record<string, string | string[]> 从 path 中提取的已解码参数字典。
    // query Record<string, string | string[]> 从 URL 的 search 部分提取的已解码查询参数的字典。
    // hash 已解码 URL 的 hash 部分。总是以 #开头。如果 URL 中没有 hash，则为空字符串。
}

impl Context {
    fn new(mut stream: TcpStream) -> Self {
        let mut buffer: ReadBuffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let header = String::from_utf8((&buffer).to_vec()).unwrap();
        let re = Regex::new(r"^(.*?) (.*?) (.*?)\r\n").unwrap();
        let caps = re.captures(&header).unwrap();
        let method = caps
            .get(1)
            .map_or("".to_string(), |m| m.as_str().to_string());
        let full_path = caps
            .get(2)
            .map_or("".to_string(), |m| m.as_str().to_string());

        let re = Regex::new(r"^(.*?)(\?.*?)?(#.*?)?$").unwrap();
        let caps = re.captures(&full_path).unwrap();
        let path = caps
            .get(1)
            .map_or("".to_string(), |m| m.as_str().to_string());
        Context {
            stream,
            method,
            full_path,
            path,
        }
    }

    pub fn string(self, status_code: u16, contents: &str) {
        let status = format!("HTTP/1.1 {} {}{}", status_code, "OK", CRLF);
        self.write(status, contents.to_owned());
    }

    // 将响应写出到流
    fn write(mut self, status: String, contents: String) {
        let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
        let server = format!("Server: Rust{}", CRLF);
        let content_length = format!("Content-Length: {}{}", contents.as_bytes().len(), CRLF);
        let response = format!(
            "{0}{1}{2}{3}{4}{5}",
            status, server, content_type, content_length, CRLF, contents
        );
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}


fn handle_404(context: Context) {
    context.string(404, "");
}

pub fn handle_connection(stream: TcpStream, router: Router) {
    let context = Context::new(stream);

    if let Some(route) = handle_router(&context, router) {
        route.1(context);
    } else {
        handle_404(context);
    }
}
