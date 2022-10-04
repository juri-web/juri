use crate::{JuriCustomError, Request};
use regex::Regex;
use std::{io::Read, net::TcpStream};

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

pub fn handle_bytes(stream: &mut TcpStream) -> Result<Request, JuriCustomError> {
    // https://www.cnblogs.com/nxlhero/p/11670942.html
    // https://rustcc.cn/article?id=2b7eb30b-61ae-4a3d-96fd-fc897ab7b1e0
    let mut temp_header_bytes = Vec::<u8>::new();
    let mut flag_body = false;
    let mut flag_request_line = false;
    const BUFFER_SIZE: usize = 1024 * 2;

    let mut request = Request::new();
    loop {
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let bytes_count = stream.read(&mut buffer).map_err(|e| JuriCustomError {
            code: 500,
            reason: e.to_string(),
        })?;
        if bytes_count == 0 {
            break;
        } else if flag_body {
            request
                .body_bytes
                .append(&mut buffer[..bytes_count].to_vec());
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
                        request
                            .body_bytes
                            .append(&mut buffer[(index + 1)..bytes_count].to_vec());
                        flag_body = true;
                        break;
                    } else {
                        // * * * * 13 / 10 * * * *
                        let header_bytes = temp_header_bytes[..temp_header_bytes.len() - 1]
                            .to_vec()
                            .clone();
                        if flag_request_line {
                            let (key, value) = handle_header_bytes(header_bytes);
                            request.header_map.insert(key, value);
                        } else {
                            let (method, full_path, version) =
                                handle_request_line_bytes(header_bytes);
                            request.method = method;
                            request.set_full_path(full_path);
                            request.version = version;
                            flag_request_line = true
                        }
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
                        request
                            .body_bytes
                            .append(&mut buffer[(index + 1)..bytes_count].to_vec());

                        flag_body = true;
                        break;
                    } else if temp_header_bytes.len() == 0 {
                        // * * * * 13 10 * * * *
                        let header_bytes = buffer[point_index..(index - 1)].to_vec().clone();
                        if flag_request_line {
                            let (key, value) = handle_header_bytes(header_bytes);
                            request.header_map.insert(key, value);
                        } else {
                            let (method, full_path, version) =
                                handle_request_line_bytes(header_bytes);
                            request.method = method;
                            request.set_full_path(full_path);
                            request.version = version;
                            flag_request_line = true
                        }
                    } else {
                        // * * / * * 13 10 * * * *
                        temp_header_bytes
                            .append(&mut buffer[point_index..(index - 1)].to_vec().clone());
                        let header_bytes = temp_header_bytes.clone();
                        if flag_request_line {
                            let (key, value) = handle_header_bytes(header_bytes);
                            request.header_map.insert(key, value);
                        } else {
                            let (method, full_path, version) =
                                handle_request_line_bytes(header_bytes);
                            request.method = method;
                            request.set_full_path(full_path);
                            request.version = version;
                            flag_request_line = true
                        }
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

    if flag_request_line == false {
        return Err(JuriCustomError {
            code: 400,
            reason: "请求方法错误".to_string(),
        });
    }
    Ok(request)
}
