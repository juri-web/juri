use crate::{Response, ResponseBody};
use serde::Serialize;
use std::collections::HashMap;

pub trait JsonResponseExt {
    fn json<T>(value: &T) -> Result<Response, crate::Error>
    where
        T: ?Sized + Serialize;
}

impl JsonResponseExt for Response {
    fn json<T>(value: &T) -> Result<Response, crate::Error>
    where
        T: ?Sized + Serialize,
    {
        let json_str = serde_json::to_string(value);

        match json_str {
            Ok(json_str) => Ok(Response {
                status_code: 200,
                headers: HashMap::from([(
                    "Content-Type".into(),
                    "application/json;charset=utf-8".into(),
                )]),
                body: ResponseBody::Text(json_str),
            }),
            Err(e) => Err(crate::Error {
                code: 401,
                reason: e.to_string(),
            }),
        }
    }
}

#[test]
fn main() {
    let response = Response::json("one").unwrap();
    if let ResponseBody::Text(text) = response.body {
        let one = String::from("\"one\"");
        assert_eq!(text, one);
    }
}
