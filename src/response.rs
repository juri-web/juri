use std::collections::HashMap;

pub const CRLF: &str = "\r\n";

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub contents: String,
    pub headers: HashMap<String, String>,
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
}

/// # Examples
///```
/// use juri::{Request, Response, ResultResponse};
///
/// fn result(flag: bool) -> ResultResponse<String> {
///     if flag {
///         Ok("Mode".to_string())
///     } else {
///         Err(Response::html_str(""))
///    }
/// }
///
/// pub fn handle_result_mode(request: Request) -> ResultResponse<Response> {
///     // Use ? quickly return Response
///     let point = result(true)?;
///
///     Ok(Response::json_str(&point))
/// }
///```
pub type ResultResponse<T> = Result<T, Response>;
