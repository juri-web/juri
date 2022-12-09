use std::collections::HashMap;
use std::fs::metadata;
use std::path::PathBuf;
mod into;
pub use into::{IntoResponse, HTTPHandler};

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
    pub headers: HashMap<String, String>,
    pub body: ResponseBody,
}

impl Response {
    pub fn json_str(contents: &str) -> Self {
        let mut response = Response {
            status_code: 200,
            headers: HashMap::new(),
            body: ResponseBody::Text(contents.to_string()),
        };
        response.headers.insert(
            "Content-Type".to_string(),
            "application/json;charset=utf-8".to_string(),
        );
        response
    }

    pub fn html_str(contents: &str) -> Self {
        let mut response = Response {
            status_code: 200,
            headers: HashMap::new(),
            body: ResponseBody::Text(contents.to_string()),
        };
        response.headers.insert(
            "Content-Type".to_string(),
            "text/html;charset=utf-8".to_string(),
        );
        response
    }

    pub fn set_status_code(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self
    }
}

impl Response {
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


impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}