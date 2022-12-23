use super::response::WSResponse;
use crate::{HTTPMethod, Request};

pub trait RequestExt {
    fn upgrader(&self) -> crate::Result<WSResponse>;
}

impl RequestExt for Request {
    fn upgrader(&self) -> crate::Result<WSResponse> {
        if self.method != HTTPMethod::GET {
            Err(crate::Error {
                code: 405,
                reason: "Method Not Allowed".to_string(),
            })?
        }

        if self.protocol_and_version != "HTTP/1.1" {
            Err(crate::Error {
                code: 406,
                reason: "Not Acceptable".to_owned(),
            })?;
        }

        if self.header("Connection") == Some("Upgrade".to_string()) {
            if self.header("Upgrade") == Some("websocket".to_string()) {
                if self.header("Sec-WebSocket-Version") == Some("13".to_string()) {
                    if self.header("Sec-WebSocket-Key") != None {
                        return Ok(WSResponse::success(self.header_map.clone()));
                    }
                }
            }
        }
        Err(crate::Error {
            code: 406,
            reason: "Not Acceptable".to_owned(),
        })?
    }
}
