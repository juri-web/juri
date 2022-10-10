use crate::{JuriCustomError, Request};
use regex::Regex;
use std::collections::HashMap;

fn handle_request_line_bytes(line_bytes: Vec<u8>) -> (String, String, String) {
    let line = String::from_utf8(line_bytes).unwrap();
    let re = Regex::new(r"^(.*?) (.*?) (.*?)$").unwrap();
    let caps = re.captures(&line).unwrap();
    let method = caps
        .get(1)
        .map_or("".to_string(), |m| m.as_str().to_string());
    let full_path = caps
        .get(2)
        .map_or("".to_string(), |m| m.as_str().to_string());
    let version = caps
        .get(3)
        .map_or("".to_string(), |m| m.as_str().to_string());
    (method, full_path, version)
}

fn handle_header_bytes(header_bytes: Vec<u8>) -> (String, String) {
    let header = String::from_utf8(header_bytes).unwrap();
    let re = Regex::new(r"^(.*?):(.*?)$").unwrap();
    let caps = re.captures(&header).unwrap();
    let key = caps
        .get(1)
        .map_or("".to_string(), |m| m.as_str().trim().to_string());
    let value = caps
        .get(2)
        .map_or("".to_string(), |m| m.as_str().trim().to_string());
    (key, value)
}

pub struct JuriStream {
    request_line: Option<(String, String, String)>,
    header_map: HashMap<String, String>,
    body_bytes: Vec<u8>,
    boundary: Option<String>,
}

impl JuriStream {
    pub fn new() -> Self {
        JuriStream {
            request_line: None,
            header_map: HashMap::new(),
            boundary: None,
            body_bytes: vec![],
        }
    }

    pub fn handle_request_header_bytes(&mut self, header_bytes: Vec<u8>) {
        if let None = self.request_line {
            self.request_line = Some(handle_request_line_bytes(header_bytes));
        } else {
            let (key, value) = handle_header_bytes(header_bytes);
            self.header_map.insert(key, value);
        }
    }

    pub fn handle_request_body_bytes(&mut self, body_bytes: &mut Vec<u8>) {
        if self.boundary.is_some() {
            self.handle_multipart_form_data(body_bytes);
        } else {
            self.body_bytes.append(body_bytes);
        }
    }

    pub fn header_end(&mut self) {
        self.is_multipart_form_data();
    }

    pub fn get_request(self) -> Result<Request, JuriCustomError> {
        let mut request = Request::new();
        let request_line = self.request_line.map_or(
            Err(JuriCustomError {
                code: 400,
                reason: "请求方法错误".to_string(),
            }),
            |v| Ok(v),
        )?;
        request.method = request_line.0;
        request.set_full_path(request_line.1);
        request.version = request_line.2;

        request.header_map = self.header_map;

        request.body_bytes = self.body_bytes;

        Ok(request)
    }
}

impl JuriStream {
    pub fn is_multipart_form_data(&mut self) -> bool {
        if let Some(content_type) = self.header_map.get("Content-Type") {
            let re = Regex::new(r"^multipart/form-data; boundary=(.*?)$").unwrap();
            if let Some(caps) = re.captures(&content_type) {
                if let Some(boundary) = caps.get(1).map(|m| m.as_str()) {
                    self.boundary = Some(boundary.to_string());
                    return true;
                }
            }
        }
        false
    }

    pub fn handle_multipart_form_data(&mut self, _body_bytes: &mut Vec<u8>) {
        // multipart/form-data
    }
}
