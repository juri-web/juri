use crate::{Config, Request, Response};
use async_std::{net::TcpStream, prelude::*};
use std::sync::Arc;

pub const CRLF: &str = "\r\n";

fn generate_response_header_bytes(
    request: &Request,
    response: &Response,
    config: &Arc<Config>,
) -> Vec<u8> {
    let status = format!("HTTP/1.1 {} {}\r\n", response.status_code, "OK");
    let server = "Server: Rust\r\n";
    let mut headers_str = format!("{}{}", status, server);

    for (key, value) in response.headers.iter() {
        headers_str.push_str(format!("{}: {}\r\n", key, value).as_str());
    }

    if request.is_keep_alive() {
        headers_str.push_str("Connection: keep-alive\r\n");
        headers_str
            .push_str(format!("Keep-Alive: timeout={}\r\n", config.keep_alive_timeout).as_str());
    } else {
        headers_str.push_str("Connection: close\r\n");
    }

    if let Some(content_length) = response.get_body_bytes_len() {
        headers_str.push_str(format!("Content-Length: {}\r\n", content_length,).as_str());
    }

    headers_str.push_str(CRLF);

    headers_str.as_bytes().to_vec()
}

pub async fn send_stream(
    stream: &mut TcpStream,
    config: &Arc<Config>,
    request: &Request,
    response: &Response,
) {
    if response.is_body_big() {
        panic!("send_stream: body big");
    } else {
        let mut bytes = generate_response_header_bytes(request, response, config);
        bytes.append(&mut response.generate_body_bytes());

        stream.write(&bytes).await.unwrap();
    }
    stream.flush().await.unwrap();
}
