use crate::{byte::stream::JuriStream, Config, Request};
use async_std::{io::ReadExt, net::TcpStream};
use std::sync::Arc;
use std::time::Duration;
mod newline;
use self::newline::Newline;

const BUFFER_SIZE: usize = 1024 * 2;

async fn read_buffer(
    stream: &mut TcpStream,
    config: &Arc<Config>,
) -> std::result::Result<(usize, Vec<u8>), crate::Error> {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let dur = Duration::from_secs(config.keep_alive_timeout);

    let bytes_count = async_std::future::timeout(dur, async {
        let bytes_count = stream.read(&mut buffer).await.map_err(|e| crate::Error {
            code: 500,
            reason: e.to_string(),
        })?;
        Ok(bytes_count)
    })
    .await
    .map_err(|e| crate::Error {
        code: 500,
        reason: e.to_string(),
    })??;

    Ok((bytes_count, buffer))
}

pub async fn handle_bytes(
    stream: &mut TcpStream,
    config: &Arc<Config>,
) -> std::result::Result<Request, crate::Error> {
    // https://www.cnblogs.com/nxlhero/p/11670942.html
    // https://rustcc.cn/article?id=2b7eb30b-61ae-4a3d-96fd-fc897ab7b1e0
    let mut juri_stream = JuriStream::new();
    let mut newline = Newline::new();

    let is_read_body_finish = loop {
        let (bytes_count, buffer) = read_buffer(stream, &config).await?;

        if bytes_count == 0 {
            break None;
        }

        let bytes = &mut buffer[..bytes_count].to_vec();
        newline.push(bytes);

        let is_exist_body = loop {
            if let Some(header_bytes) = newline.next() {                                                
                if header_bytes.is_empty() {
                    break true;
                }
                juri_stream.handle_request_header_bytes(header_bytes);
            } else {
                break false;
            }
        };  

        if is_exist_body {
            break Some(bytes_count < BUFFER_SIZE);
        }

        if bytes_count < BUFFER_SIZE {
            break None;
        }
    };

    juri_stream.header_end();

    if let Some(is_read_body_finish) = is_read_body_finish {
        if let Some(body_bytes) = &mut newline.get_residue_bytes() {
            juri_stream.handle_request_body_bytes(body_bytes).await;
        }
        if !is_read_body_finish {
            loop {
                let (bytes_count, buffer) = read_buffer(stream, &config).await?;
                if bytes_count == 0 {
                    break;
                }
                let body_bytes = &mut buffer[..bytes_count].to_vec();
                juri_stream.handle_request_body_bytes(body_bytes).await;
                if bytes_count < BUFFER_SIZE {
                    break;
                }
            }
        } else if let Some(content_length) = juri_stream.header_map.get("Content-Length") {
            // 处理读取 header 时，读取数据大小小于缓冲区大小，但是 header 已经读取完毕
            if let Ok(_body_length) = content_length.parse::<usize>() {
                loop {
                    let (bytes_count, buffer) = read_buffer(stream, &config).await?;
                    if bytes_count == 0 {
                        break;
                    }
                    
                    let body_bytes = &mut buffer[..bytes_count].to_vec();
                    juri_stream.handle_request_body_bytes(body_bytes).await;

                    if bytes_count < BUFFER_SIZE {
                        break;
                    }
                }
            }
        }
    }

    juri_stream.get_request()
}
