use std::collections::HashMap;

#[derive(Debug, Clone)]
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
            "application/json;charset=utf-8".to_string(),
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
    pub fn is_body_big(&self) -> bool {
        false
    }

    pub fn get_body_bytes_len(&self) -> Option<usize> {
        Some(self.contents.as_bytes().len())
    }

    pub fn generate_body_bytes(&self) -> Vec<u8> {
        self.contents.as_bytes().to_vec()
    }
}
