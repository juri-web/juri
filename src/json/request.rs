use serde_json::Value;

use crate::Request;

pub trait RequestExt {
    fn json_value(&self) -> serde_json::Result<Value>;
}

impl RequestExt for Request {
    fn json_value(&self) -> serde_json::Result<Value> {
        serde_json::from_slice(&self.body_bytes)
    }
}
