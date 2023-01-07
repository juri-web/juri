use crate::Request;
use serde_json::Value;

pub trait JsonRequestExt {
    fn json_value(&self) -> Result<Value, crate::Error>;
}

impl JsonRequestExt for Request {
    fn json_value(&self) -> Result<Value, crate::Error> {
        match serde_json::from_slice(&self.body_bytes) {
            Ok(json_value) => Ok(json_value),
            Err(e) => Err(crate::Error {
                code: 401,
                reason: e.to_string(),
            }),
        }
    }
}
