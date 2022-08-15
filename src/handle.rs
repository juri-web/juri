use regex::Regex;

use super::router::{handle_router, Router};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};
const CRLF: &str = "\r\n";
/**1KB */
type ReadBuffer = [u8; 1024];

pub struct Context {
    stream: TcpStream,
    pub method: String,
    pub full_path: String,
    pub path: String,
    query_str: String,
    pub hash: String, // params Record<string, string | string[]> 从 path 中提取的已解码参数字典。
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

        let (path, query_str, hash) = handle_full_path(&full_path);

        Context {
            stream,
            method,
            full_path,
            path,
            query_str,
            hash,
        }
    }

    pub fn query(&self, key: &str, default: &str) -> String {
        if self.query_str.is_empty() {
            default.to_string()
        } else {
            let re = Regex::new(&format!(r"[\?|\&]{}=(.*?)(\&|$)", key)).unwrap();
            let caps = re.captures(&self.query_str);
            if let Some(caps) = caps {
                caps.get(1)
                    .map_or(default.to_string(), |m| m.as_str().to_string())
            } else {
                default.to_string()
            }
        }
    }

    pub fn string(self, status_code: u16, contents: &str) {
        let status = format!("HTTP/1.1 {} {}{}", status_code, "OK", CRLF);
        let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
        let server = format!("Server: Rust{}", CRLF);
        let content_length = format!("Content-Length: {}{}", contents.as_bytes().len(), CRLF);
        let response = format!(
            "{0}{1}{2}{3}{4}{5}",
            status, server, content_type, content_length, CRLF, contents
        );
        self.write(response.as_bytes());
    }

    pub fn json(self, status_code: u16, contents: &str) {
        let status = format!("HTTP/1.1 {} {}{}", status_code, "OK", CRLF);
        let content_type = format!("Content-Type: application/json;charset=utf-8{}", CRLF);
        let server = format!("Server: Rust{}", CRLF);
        let content_length = format!("Content-Length: {}{}", contents.as_bytes().len(), CRLF);
        let response = format!(
            "{0}{1}{2}{3}{4}{5}",
            status, server, content_type, content_length, CRLF, contents
        );
        self.write(response.as_bytes());
    }

    // 将响应写出到流
    fn write(mut self, buf: &[u8]) {
        self.stream.write(buf).unwrap();
        self.stream.flush().unwrap();
    }
}

fn handle_full_path(full_path: &String) -> (String, String, String) {
    let re = Regex::new(r"^(.*?)(\?.*?)?(#.*?)?$").unwrap();
    let caps = re.captures(&full_path).unwrap();
    let path = caps
        .get(1)
        .map_or("".to_string(), |m| m.as_str().to_string());
    let query_str = caps
        .get(2)
        .map_or("".to_string(), |m| m.as_str().to_string());
    let hash = caps
        .get(3)
        .map_or("".to_string(), |m| m.as_str().to_string());
    (path, query_str, hash)
}

fn handle_404(context: Context) {
    context.string(404, "");
}

pub fn handle_connection(stream: TcpStream, router: Arc<Router>) {
    let context = Context::new(stream);

    if let Some(fun) = handle_router(&context, router) {
        fun(context);
    } else {
        handle_404(context);
    }
}
