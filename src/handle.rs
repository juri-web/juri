use super::run::{Route, Router};
use std::io::prelude::*;
use std::net::TcpStream;
const CRLF: &str = "\r\n";
type ReadBuffer = [u8; 1024];

pub struct Context {
    stream: TcpStream,
}

impl Context {
    fn new(stream: TcpStream) -> Self {
        Context { stream }
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

pub fn handle_connection(mut stream: TcpStream, router: Router) {
    let mut buffer: ReadBuffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let _matched = |route: &str| matched(&buffer, route);

    // 路由处理
    let mut routes = Vec::<Route>::new();
    if buffer.starts_with("GET".as_bytes()) {
        routes = router.get;
    } else if buffer.starts_with("POST".as_bytes()) {
        routes = router.post;
    }

    let context = Context::new(stream);
    let len = routes.len();
    for i in 0..(len + 1) {
        if i >= len {
            context.string(404, "");
            return;
        }

        let route = routes.get(i);
        if let Some(route) = route {
            if _matched(&route.0) {
                route.1(context);
                return;
            }
        }
    }
}

// 路由匹配
fn matched(buffer: &ReadBuffer, route: &str) -> bool {
    let s = format!("GET {} HTTP/1.1{}", route, CRLF);
    buffer.starts_with(s.as_bytes())
}

// 将响应写出到流
// fn write(stream: &mut TcpStream, contents: String, status: String) {
//     let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
//     let server = format!("Server: Rust{}", CRLF);
//     let content_length = format!("Content-Length: {}{}", contents.as_bytes().len(), CRLF);
//     let response = format!(
//         "{0}{1}{2}{3}{4}{5}",
//         status, server, content_type, content_length, CRLF, contents
//     );
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }
