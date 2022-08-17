use std::{collections::HashMap, io::Write, net::TcpStream};

const CRLF: &str = "\r\n";

pub struct Response {
    status_code: u16,
    contents: String,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn json_str(contents: &str) -> Self {
        let mut response = Response {
            status_code: 200,
            contents: contents.to_string(),
            headers: HashMap::new(),
        };
        response.headers.insert(
            "Content-Type".to_string(),
            format!("application/json;charset=utf-8{}", CRLF),
        );
        response
    }

    pub fn html_str(contents: &str) -> Self {
        let mut response = Response {
            status_code: 200,
            contents: contents.to_string(),
            headers: HashMap::new(),
        };
        response.headers.insert(
            "Content-Type".to_string(),
            format!("text/html;charset=utf-8{}", CRLF),
        );
        response
    }

    pub fn set_status_code(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn get_response_str(self) -> String {
        let status = format!("HTTP/1.1 {} {}{}", self.status_code, "OK", CRLF);
        let server = format!("Server: Rust{}", CRLF);
        let content_length = format!("Content-Length: {}{}", self.contents.as_bytes().len(), CRLF);
        let mut headers_str = format!("{0}{1}{2}", status, server, content_length);
        for (key, value) in self.headers {
            headers_str.push_str(format!("{}: {}", key, value).as_str());
        }
        format!("{0}{1}{2}", headers_str, CRLF, self.contents)
    }

    pub fn write(self, stream: &mut TcpStream) {
        let response_str = self.get_response_str();
        stream.write(response_str.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
