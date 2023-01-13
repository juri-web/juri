mod form_data;
mod line;
mod stream;

use crate::{Config, Request};
use async_std::{future::timeout, io::ReadExt, net::TcpStream};
pub use form_data::FormData;
use line::Line;
use std::{sync::Arc, time::Duration};
use stream::ReadStream;
const BUFFER_SIZE: usize = 1024 * 2;

async fn read_buffer(
    stream: &mut TcpStream,
    config: &Arc<Config>,
) -> Result<(usize, Vec<u8>), crate::Error> {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let dur = Duration::from_secs(config.keep_alive_timeout);

    let bytes_count = timeout(dur, async {
        stream.read(&mut buffer).await.map_err(|e| crate::Error {
            code: 500,
            reason: e.to_string(),
        })
    })
    .await
    .map_err(|e| crate::Error {
        code: 500,
        reason: e.to_string(),
    })??;

    Ok((bytes_count, buffer))
}

pub async fn read_request(
    stream: &mut TcpStream,
    config: &Arc<Config>,
) -> std::result::Result<Request, crate::Error> {
    // https://www.cnblogs.com/nxlhero/p/11670942.html
    // https://rustcc.cn/article?id=2b7eb30b-61ae-4a3d-96fd-fc897ab7b1e0
    let mut read_stream = ReadStream::default();
    let mut line = Line::new();

    // Body: None 读取完成, True 可能读取完成, False 未完成
    let is_read_body_finish = loop {
        let (bytes_count, mut buffer) = read_buffer(stream, config).await?;

        if bytes_count == 0 {
            break None;
        }

        line.push(&mut buffer);

        let is_exist_body = loop {
            let Some(header_bytes) = line.next() else {
                break false;
            };

            if header_bytes.is_empty() {
                break true;
            }

            read_stream.write_header(header_bytes)?;
        };

        if is_exist_body {
            break Some(bytes_count < BUFFER_SIZE);
        }

        if bytes_count < BUFFER_SIZE {
            break None;
        }
    };

    read_stream.header_end();

    if let Some(is_read_body_finish) = is_read_body_finish {
        let mut already_read_body_length: usize = 0;
        if let Some(body_bytes) = &mut line.get_residue_bytes() {
            already_read_body_length = body_bytes.len();
            read_stream.write_body(body_bytes).await?;
        }
        if !is_read_body_finish {
            loop {
                let (bytes_count, buffer) = read_buffer(stream, config).await?;
                if bytes_count == 0 {
                    break;
                }
                let body_bytes = &mut buffer[..bytes_count].to_vec();
                read_stream.write_body(body_bytes).await?;
                if bytes_count < BUFFER_SIZE {
                    break;
                }
            }
        } else if let Some(content_length) = read_stream.headers.get("Content-Length") {
            if let Some(content_length) = content_length.last() {
                // 处理读取 header 时，读取数据大小小于缓冲区大小，但是 header 已经读取完毕
                if let Ok(content_length) = content_length.parse::<usize>() {
                    if content_length != 0 && content_length > already_read_body_length {
                        loop {
                            let (bytes_count, buffer) = read_buffer(stream, config).await?;
                            if bytes_count == 0 {
                                break;
                            }

                            let body_bytes = &mut buffer[..bytes_count].to_vec();
                            read_stream.write_body(body_bytes).await?;

                            if bytes_count < BUFFER_SIZE {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    read_stream.get_request()
}
