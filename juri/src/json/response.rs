use serde::Serialize;
use crate::Response;

pub trait ResponseExt {
    fn json<T>(value: &T) -> Self
    where
        T: ?Sized + Serialize;
}

impl ResponseExt for Response {
    fn json<T>(value: &T) -> Self
    where
        T: ?Sized + Serialize,
    {
        let json_str = serde_json::to_string(value).unwrap();
        Response::json_str(json_str.as_str())
    }
}
