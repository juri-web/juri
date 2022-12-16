use super::response::WSResponse;
use crate::{HTTPMethod, Request};

pub trait WSRequest {
    fn upgrader(&self) -> crate::Result<WSResponse>;
}

impl WSRequest for Request {
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

        if let Some(connection) = self.header("Connection") {
            if connection == "Upgrade" {
                if let Some(upgrade) = self.header("Upgrade") {
                    if upgrade == "websocket" {
                        return Ok(WSResponse::success());
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
