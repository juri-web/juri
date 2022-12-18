use super::{
    frame::{Frame, OpCode},
    message::{IncompleteMessage, IncompleteMessageType, Message},
};
use async_std::net::TcpStream;

/// 参考 https://github.com/snapview/tungstenite-rs
pub struct WSStream {
    stream: TcpStream,
    incomplete: Option<IncompleteMessage>,
}

impl WSStream {
    pub fn new(stream: TcpStream) -> Self {
        WSStream {
            stream,
            incomplete: None,
        }
    }
}

impl WSStream {
    pub async fn read(&mut self) -> Result<Message, crate::Error> {
        loop {
            let mut frame = Frame::read_frame(&mut self.stream).await?;
            frame.apply_mask();
            match frame.header.opcode {
                OpCode::Continue => {
                    if frame.header.fin {
                        return Ok(self.incomplete.take().unwrap().complete());
                    } else {
                        self.incomplete.as_mut().unwrap().extend(frame.payload);
                    }
                }
                OpCode::Text | OpCode::Binary => {
                    if frame.header.fin {
                        return match frame.header.opcode {
                            OpCode::Text => {
                                Ok(Message::Text(String::from_utf8(frame.payload).unwrap()))
                            }
                            OpCode::Binary => Ok(Message::Binary(frame.payload)),
                            _ => panic!("Bug: message is not text nor binary"),
                        };
                    } else {
                        let msg = {
                            let message_type = match frame.header.opcode {
                                OpCode::Text => IncompleteMessageType::Text,
                                OpCode::Binary => IncompleteMessageType::Binary,
                                _ => panic!("Bug: message is not text nor binary"),
                            };
                            let mut m = IncompleteMessage::new(message_type);
                            m.extend(frame.payload);
                            m
                        };
                        self.incomplete = Some(msg);
                    }
                }
                OpCode::Close => return Ok(Message::Close),
                OpCode::Ping => return Ok(Message::Ping(frame.payload)),
                OpCode::Pong => return Ok(Message::Pong(frame.payload)),
            }
        }
    }
}
