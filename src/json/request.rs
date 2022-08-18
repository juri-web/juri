use crate::Request;

pub trait RequestExt {
    fn json(self);
}

impl RequestExt for Request {
    fn json(self) {
        serde_json::from_slice(&self.body_bytes).unwrap()
    }
}
