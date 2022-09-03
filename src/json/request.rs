use serde_json::Value;

use crate::Request;

pub trait RequestExt {
    fn json_value(self) -> Value;
}

impl RequestExt for Request {
    fn json_value(self) -> Value {
        serde_json::from_slice(&self.body_bytes).unwrap()
    }
}
