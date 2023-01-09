use async_std::{io::ReadExt, net::TcpStream};
use byteorder::{NetworkEndian, ReadBytesExt};
use std::{
    io::{Cursor, Read},
    time::Duration,
};

use super::WSConfig;

#[derive(Clone)]
pub enum OpCode {
    /// 0x0
    Continue,
    /// 0x1
    Text,
    /// 0x2
    Binary,
    /// 0x8
    Close,
    /// 0x9
    Ping,
    /// 0xa
    Pong,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> OpCode {
        use self::OpCode::*;
        match byte {
            0 => Continue,
            1 => Text,
            2 => Binary,
            8 => Close,
            9 => Ping,
            10 => Pong,
            _ => panic!("Bug: OpCode out of range"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(opcode: OpCode) -> Self {
        use self::OpCode::*;
        match opcode {
            Continue => 0,
            Text => 1,
            Binary => 2,
            Close => 8,
            Ping => 9,
            Pong => 10,
        }
    }
}

pub struct FrameHeader {
    pub fin: bool,
    rsv1: bool,
    rsv2: bool,
    rsv3: bool,
    pub opcode: OpCode,

    #[allow(dead_code)]
    payload_length: u64,
    pub masking_key: Option<[u8; 4]>,
}

impl Default for FrameHeader {
    fn default() -> Self {
        FrameHeader {
            fin: true,
            rsv1: false,
            rsv2: false,
            rsv3: false,
            opcode: OpCode::Close,
            payload_length: 0,
            masking_key: None,
        }
    }
}

impl FrameHeader {
    fn parse(cursor: &mut Cursor<Vec<u8>>) -> std::result::Result<FrameHeader, crate::Error> {
        let (first, second) = {
            let mut head = [0u8; 2];
            let count = cursor.read(&mut head).map_err(|e| crate::Error {
                code: 400,
                reason: e.to_string(),
            })?;
            if count != 2 {
                return Err(crate::Error {
                    code: 400,
                    reason: "Web Sccoket parse FIN failed".to_string(),
                });
            }
            (head[0], head[1])
        };

        let fin = first & 0x80 != 0;

        let rsv1 = first & 0x40 != 0;
        let rsv2 = first & 0x20 != 0;
        let rsv3 = first & 0x10 != 0;

        let opcode = OpCode::from(first & 0x0F);

        let mask = second & 0x80 != 0;

        let payload_length: u64 = {
            let length_byte = second & 0x7F;

            match length_byte {
                127 => cursor
                    .read_u64::<NetworkEndian>()
                    .map_err(|e| crate::Error {
                        code: 400,
                        reason: e.to_string(),
                    })?,
                126 => cursor
                    .read_u16::<NetworkEndian>()
                    .map_err(|e| crate::Error {
                        code: 400,
                        reason: e.to_string(),
                    })?
                    .into(),
                length => length.into(),
            }
        };
        let masking_key = if mask {
            let mut masking_key = [0u8; 4];
            let count = cursor.read(&mut masking_key).map_err(|e| crate::Error {
                code: 400,
                reason: e.to_string(),
            })?;
            if count != 4 {
                return Err(crate::Error {
                    code: 400,
                    reason: "Web Sccoket parse masking key failed".to_string(),
                });
            }
            Some(masking_key)
        } else {
            None
        };
        Ok(FrameHeader {
            fin,
            rsv1,
            rsv2,
            rsv3,
            opcode,

            payload_length,
            masking_key,
        })
    }

    fn format(&self, payload_length: u64) -> Vec<u8> {
        let opcode: u8 = self.opcode.clone().into();
        let mut header_bytes = vec![];
        let one = {
            opcode
                | if self.fin { 0x80 } else { 0 }
                | if self.rsv1 { 0x40 } else { 0 }
                | if self.rsv2 { 0x20 } else { 0 }
                | if self.rsv3 { 0x10 } else { 0 }
        };

        header_bytes.push(one);
        if payload_length < 126 {
            header_bytes.push(payload_length as u8);
        } else if payload_length < 65536 {
            header_bytes.push(0x7e);
            let mut bytes = (payload_length as u16).to_le_bytes().to_vec();
            header_bytes.append(&mut bytes);
        } else {
            header_bytes.push(0x7f);
            let mut bytes = payload_length.to_le_bytes().to_vec();
            header_bytes.append(&mut bytes);
        }

        header_bytes
    }
}

const BUFFER_SIZE: usize = 1024 * 2;

pub struct Frame {
    pub header: FrameHeader,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn text(text: String) -> Frame {
        Frame {
            header: FrameHeader {
                opcode: OpCode::Text,
                ..FrameHeader::default()
            },
            payload: text.as_bytes().to_vec(),
        }
    }

    pub fn binary(binary: Vec<u8>) -> Frame {
        Frame {
            header: FrameHeader {
                opcode: OpCode::Binary,
                ..FrameHeader::default()
            },
            payload: binary,
        }
    }

    pub fn pong(payload: Vec<u8>) -> Frame {
        Frame {
            header: FrameHeader {
                opcode: OpCode::Pong,
                ..FrameHeader::default()
            },
            payload,
        }
    }

    pub fn ping(payload: Vec<u8>) -> Frame {
        Frame {
            header: FrameHeader {
                opcode: OpCode::Ping,
                ..FrameHeader::default()
            },
            payload,
        }
    }

    pub fn close() -> Frame {
        Frame {
            header: FrameHeader::default(),
            payload: vec![],
        }
    }
}

impl Frame {
    pub async fn read_frame(
        stream: &mut TcpStream,
        config: &WSConfig,
    ) -> std::result::Result<Frame, crate::Error> {
        let frame_bytes = Frame::read_stream(stream, config).await?;
        Frame::parse(frame_bytes).await
    }

    async fn read_stream(
        stream: &mut TcpStream,
        config: &WSConfig,
    ) -> std::result::Result<Vec<u8>, crate::Error> {
        let mut frame_bytes = vec![];
        loop {
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

            if bytes_count == 0 {
                break;
            }

            if bytes_count < BUFFER_SIZE {
                frame_bytes.append(&mut buffer[..bytes_count].to_vec());
                break;
            } else {
                frame_bytes.append(&mut buffer);
            }
        }

        Ok(frame_bytes)
    }

    async fn parse(bytes: Vec<u8>) -> std::result::Result<Frame, crate::Error> {
        let mut cursor = Cursor::new(bytes);
        let header = FrameHeader::parse(&mut cursor)?;
        let mut payload = vec![];
        cursor.read_to_end(&mut payload).unwrap();
        Ok(Frame { header, payload })
    }

    pub fn format(&self) -> Vec<u8> {
        let mut bytes = self.header.format(self.payload.len().try_into().unwrap());
        bytes.append(&mut self.payload.clone());
        bytes
    }

    /// https://www.rfc-editor.org/rfc/rfc6455#section-5.3
    pub fn apply_mask(&mut self) {
        if let Some(masking_key) = self.header.masking_key {
            for (i, original_octet_i) in self.payload.iter_mut().enumerate() {
                let j = i % 4;
                *original_octet_i ^= masking_key[j];
            }
        }
    }
}

#[test]
fn test() {
    let a: u16 = 12;
    println!("{:#?}", a.to_be_bytes());

    let mut a: Vec<u8> = vec![];
    a.append(&mut vec![1, 2]);
    println!("{:#?}", a);
}
