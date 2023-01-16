mod into;

pub use into::HTTPHandler;
use std::fs::metadata;
use std::path::PathBuf;

use crate::http::{Cookie, Headers};

#[derive(Debug, Clone)]
pub enum ResponseBody {
    Text(String),
    Path(PathBuf),
    None,
}

pub enum ResponseBodyByte {
    All(Vec<u8>),
    File(PathBuf),
    None,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u16,
    pub headers: Headers,
    pub body: ResponseBody,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status_code: 200,
            headers: Default::default(),
            body: ResponseBody::None,
        }
    }
}

impl Response {
    pub fn set_status_code(&mut self, status_code: u16) -> &mut Self {
        self.status_code = status_code;
        self
    }

    pub fn set_cookie(&mut self, cookie: Cookie) -> &mut Self {
        self.headers.insert("Set-Cookie", &cookie.to_string());
        self
    }

    pub fn get_body_bytes_len(&self) -> Option<usize> {
        match &self.body {
            ResponseBody::Text(text) => Some(text.as_bytes().len()),
            ResponseBody::Path(path) => {
                let file_metadata = metadata(path).unwrap();
                Some(file_metadata.len().try_into().unwrap())
            }
            ResponseBody::None => None,
        }
    }

    pub fn get_body_bytes(&self) -> ResponseBodyByte {
        match &self.body {
            ResponseBody::Text(text) => ResponseBodyByte::All(text.as_bytes().to_vec()),
            ResponseBody::Path(path) => ResponseBodyByte::File(path.clone()),
            ResponseBody::None => ResponseBodyByte::None,
        }
    }
}

impl Response {
    pub fn html(content: &str) -> Response {
        let mut headers = Headers::default();
        headers.insert("Content-Type", "text/html;charset=utf-8");
        Response {
            status_code: 200,
            headers,
            body: ResponseBody::Text(content.to_string()),
        }
    }

    pub fn html_file(file_path: PathBuf) -> Result<Response, crate::Error> {
        if file_path.exists() && file_path.is_file() {
            return Err(crate::Error {
                code: 401,
                reason: format!("File cannot be found, FilePath: {:?}", file_path.to_str()),
            });
        }
        Ok(Response {
            body: crate::ResponseBody::Path(file_path),
            ..Default::default()
        })
    }
}
