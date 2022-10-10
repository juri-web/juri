use crate::{JuriCustomError, Request};
use regex::Regex;
use std::{collections::HashMap, io::Read, net::TcpStream};

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

struct JuriStream {
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

    fn handle_request_header_bytes(&mut self, header_bytes: Vec<u8>) {
        if let None = self.request_line {
            self.request_line = Some(handle_request_line_bytes(header_bytes));
        } else {
            let (key, value) = handle_header_bytes(header_bytes);
            self.header_map.insert(key, value);
        }
    }

    fn handle_request_body_bytes(&mut self, body_bytes: &mut Vec<u8>) {
        if self.boundary.is_some() {
            self.handle_form_data(body_bytes);
        } else {
            self.body_bytes.append(body_bytes);
        }
    }

    fn header_end(&mut self) {
        self.is_form_data();
    }

    fn get_request(self) -> Result<Request, JuriCustomError> {
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
    pub fn is_form_data(&mut self) -> bool {
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

    pub fn handle_form_data(&mut self, _body_bytes: &mut Vec<u8>) {
        // multipart/form-data
    }
}

pub fn handle_bytes(stream: &mut TcpStream) -> Result<Request, JuriCustomError> {
    // https://www.cnblogs.com/nxlhero/p/11670942.html
    // https://rustcc.cn/article?id=2b7eb30b-61ae-4a3d-96fd-fc897ab7b1e0
    let mut temp_header_bytes = Vec::<u8>::new();
    let mut flag_body = false;
    const BUFFER_SIZE: usize = 1024 * 2;

    let mut juri_stream = JuriStream::new();

    loop {
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let bytes_count = stream.read(&mut buffer).map_err(|e| JuriCustomError {
            code: 500,
            reason: e.to_string(),
        })?;
        if bytes_count == 0 {
            break;
        } else if flag_body {
            let body_bytes = &mut buffer[..bytes_count].to_vec();
            juri_stream.handle_request_body_bytes(body_bytes);
        } else {
            let mut flag_n = false;
            let mut flag_r = false;
            let mut point_index = 0;
            for (index, value) in buffer.iter().enumerate() {
                if index == 0
                    && *value == 10
                    && temp_header_bytes.len() >= 1
                    && temp_header_bytes.last() == Some(&13)
                {
                    if temp_header_bytes.len() == 1 {
                        // 13 / 10 * * * *
                        flag_body = true;
                        juri_stream.header_end();
                        let body_bytes = &mut buffer[(index + 1)..bytes_count].to_vec();
                        juri_stream.handle_request_body_bytes(body_bytes);
                        break;
                    } else {
                        // * * * * 13 / 10 * * * *
                        let header_bytes = temp_header_bytes[..temp_header_bytes.len() - 1]
                            .to_vec()
                            .clone();
                        juri_stream.handle_request_header_bytes(header_bytes);
                        temp_header_bytes.clear();
                        point_index = index + 1;
                        continue;
                    }
                }

                if flag_n {
                    if *value == 10 {
                        flag_r = true;
                    } else {
                        flag_n = false;
                    }
                }
                if *value == 13 {
                    flag_n = true;
                }
                if flag_n && flag_r {
                    if index == point_index + 1 {
                        if bytes_count == index + 1 {
                            break;
                        }
                        // * * / * * 13 10 * * * * or 13 10 * * * *
                        flag_body = true;
                        juri_stream.header_end();
                        let body_bytes = &mut buffer[(index + 1)..bytes_count].to_vec();
                        juri_stream.handle_request_body_bytes(body_bytes);
                        break;
                    } else if temp_header_bytes.len() == 0 {
                        // * * * * 13 10 * * * *
                        let header_bytes = buffer[point_index..(index - 1)].to_vec().clone();
                        juri_stream.handle_request_header_bytes(header_bytes);
                    } else {
                        // * * / * * 13 10 * * * *
                        temp_header_bytes
                            .append(&mut buffer[point_index..(index - 1)].to_vec().clone());
                        let header_bytes = temp_header_bytes.clone();
                        juri_stream.handle_request_header_bytes(header_bytes);
                        temp_header_bytes.clear();
                    }
                    point_index = index + 1;
                    flag_n = false;
                    flag_r = false;
                }
            }
            if !flag_body {
                if point_index == 0 {
                    temp_header_bytes.append(&mut buffer.to_vec().clone())
                } else if point_index != buffer.len() {
                    temp_header_bytes.append(&mut buffer[point_index..].to_vec().clone());
                }
            }
        }
        if bytes_count < BUFFER_SIZE {
            break;
        }
    }

    juri_stream.get_request()
}
