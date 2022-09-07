use std::{net::TcpStream, io::{self, Read}};

pub fn handle_bytes(stream: &mut TcpStream) -> io::Result<(Vec<Vec<u8>>, Vec<u8>)> {
    // https://www.cnblogs.com/nxlhero/p/11670942.html
    // https://rustcc.cn/article?id=2b7eb30b-61ae-4a3d-96fd-fc897ab7b1e0
    let mut headers_bytes = Vec::<Vec<u8>>::new();
    let mut body_bytes = Vec::<u8>::new();
    let mut temp_header_bytes = Vec::<u8>::new();
    let mut flag_body = false;
    const BUFFER_SIZE: usize = 1024 * 2;
    loop {
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count == 0 {
            break;
        } else if flag_body {
            body_bytes.append(&mut buffer[..bytes_count].to_vec());
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
                        body_bytes.append(&mut buffer[(index + 1)..bytes_count].to_vec());
                        flag_body = true;
                        break;
                    } else {
                        // * * * * 13 / 10 * * * *
                        headers_bytes.push(
                            temp_header_bytes[..temp_header_bytes.len() - 1]
                                .to_vec()
                                .clone(),
                        );
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
                        body_bytes.append(&mut buffer[(index + 1)..bytes_count].to_vec());

                        flag_body = true;
                        break;
                    } else if temp_header_bytes.len() == 0 {
                        // * * * * 13 10 * * * *
                        headers_bytes.push(buffer[point_index..(index - 1)].to_vec().clone());
                    } else {
                        // * * / * * 13 10 * * * *
                        temp_header_bytes
                            .append(&mut buffer[point_index..(index - 1)].to_vec().clone());
                        headers_bytes.push(temp_header_bytes.clone());
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
    Ok((headers_bytes, body_bytes))
}
