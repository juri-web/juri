mod mime;
mod status_code;

use self::status_code::status_code_to_text;
use crate::response::ResponseBodyByte;
use crate::{Config, Request, Response};
use async_std::fs::File;
use async_std::{net::TcpStream, prelude::*};
use mime::extension_to_mime;
use std::sync::Arc;

pub const CRLF: &str = "\r\n";
const BUFFER_SIZE: usize = 1024 * 2;

fn generate_response_header_bytes(
    request: Option<&Request>,
    response: &Response,
    config: &Arc<Config>,
) -> Vec<u8> {
    let status = format!(
        "HTTP/1.1 {} {}\r\n",
        response.status_code,
        status_code_to_text(response.status_code)
    );
    let server = "Server: Rust\r\n";
    let mut headers_str = format!("{}{}", status, server);

    let mut headers = response.headers.clone();

    if let Some(request) = request {
        if request.is_keep_alive() {
            headers.insert("Connection", "keep-alive");
            headers.insert(
                "Keep-Alive",
                &format!("timeout={}", config.keep_alive_timeout),
            );
        } else if response.headers.get("Connection").is_none() {
            headers.insert("Connection", "close");
        }
    }

    if let ResponseBodyByte::File(file_path) = response.get_body_bytes() {
        if let Some(extension) = file_path.extension() {
            if let Some(extension) = extension.to_str() {
                if let Some(content_type) = extension_to_mime(extension) {
                    headers.insert("Content-Type", content_type);
                }
            }
        }
    }

    if let Some(content_length) = response.get_body_bytes_len() {
        headers.insert("Content-Length", &content_length.to_string());
    }

    for (key, values) in headers.iter() {
        for value in values.iter() {
            headers_str.push_str(format!("{}: {}\r\n", key, value).as_str());
        }
    }

    headers_str.push_str(CRLF);

    headers_str.as_bytes().to_vec()
}

pub async fn send_stream(
    stream: &mut TcpStream,
    config: &Arc<Config>,
    request: Option<&Request>,
    response: &Response,
) {
    let mut bytes = generate_response_header_bytes(request, response, config);
    match response.get_body_bytes() {
        ResponseBodyByte::All(mut body_bytes) => {
            bytes.append(&mut body_bytes);
            stream.write(&bytes).await.unwrap();
        }
        ResponseBodyByte::File(path) => {
            let mut file = File::open(path).await.unwrap();
            let mut buffer = vec![0u8; BUFFER_SIZE];

            stream.write(&bytes).await.unwrap();
            loop {
                let bytes_count = file.read(&mut buffer).await.unwrap();
                if bytes_count == 0 {
                    break;
                }

                stream.write(&buffer).await.unwrap();

                if bytes_count < BUFFER_SIZE {
                    break;
                }
            }
        }
        ResponseBodyByte::None => {
            stream.write(&bytes).await.unwrap();
        }
    }
    stream.flush().await.unwrap();
}
