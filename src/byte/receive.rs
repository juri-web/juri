use crate::{byte::stream::JuriStream, Config, Request};
use async_std::{io::ReadExt, net::TcpStream};
use std::sync::Arc;
use std::time::Duration;
const BUFFER_SIZE: usize = 1024 * 2;

async fn read_buffer(
    stream: &mut TcpStream,
    config: &Arc<Config>,
) -> std::result::Result<(usize, Vec<u8>), crate::Error> {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let dur = Duration::from_secs(config.keep_alive_timeout);

    let bytes_count = async_std::future::timeout(dur, async {
        let bytes_count = stream
            .read(&mut buffer)
            .await
            .map_err(|e| crate::Error {
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
    let mut temp_header_bytes = Vec::<u8>::new();
    let mut temp_bytes = Vec::<u8>::new();
    let mut juri_stream = JuriStream::new();

    let is_exist_body = loop {
        let (bytes_count, buffer) = read_buffer(stream, &config).await?;

        if bytes_count == 0 {
            break false;
        } else {
            let mut flag_n = false;
            let mut flag_r = false;
            let mut point_index = 0;
            let mut buffer_iter = buffer.iter().enumerate();
            let is_exist_body = loop {
                if let Some((index, value)) = buffer_iter.next() {
                    if index == 0
                        && *value == 10
                        && temp_header_bytes.len() >= 1
                        && temp_header_bytes.last() == Some(&13)
                    {
                        if temp_header_bytes.len() == 1 {
                            // 13 / 10 * * * *
                            temp_bytes.clear();
                            let body_bytes = &mut buffer[(index + 1)..bytes_count].to_vec();
                            temp_bytes.append(body_bytes);
                            break true;
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

                    if flag_r {
                        if *value == 10 {
                            flag_n = true;
                        } else {
                            flag_r = false;
                        }
                    }
                    if *value == 13 {
                        flag_r = true;
                    }
                    if flag_n && flag_r {
                        if index == point_index + 1 {
                            if bytes_count == index + 1 {
                                break false;
                            }
                            // * * / * * 13 10 * * * * or 13 10 * * * *
                            let body_bytes = &mut buffer[(index + 1)..bytes_count].to_vec();
                            temp_bytes.append(body_bytes);
                            break true;
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
                } else {
                    break false;
                }
            };

            if is_exist_body {
                break true;
            }

            if point_index == 0 {
                temp_header_bytes.append(&mut buffer.to_vec().clone())
            } else if point_index != buffer.len() {
                temp_header_bytes.append(&mut buffer[point_index..].to_vec().clone());
            }
        }
        if bytes_count < BUFFER_SIZE {
            break false;
        }
    };

    juri_stream.header_end();

    if is_exist_body {
        let body_bytes = &mut temp_bytes.clone();
        juri_stream.handle_request_body_bytes(body_bytes).await;
        loop {
            let (bytes_count, buffer) = read_buffer(stream, &config).await?;
            if bytes_count == 0 {
                break;
            } else {
                let body_bytes = &mut buffer[..bytes_count].to_vec();
                juri_stream.handle_request_body_bytes(body_bytes).await;
            }
            if bytes_count < BUFFER_SIZE {
                break;
            }
        }
    }

    juri_stream.get_request()
}
