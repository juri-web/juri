use crate::{http::Headers, Response, ResponseBody};
use serde::Serialize;

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
            Ok(json_str) => {
                let mut headers = Headers::default();
                headers.insert("Content-Type", "application/json;charset=utf-8");
                Ok(Response {
                    status_code: 200,
                    headers,
                    body: ResponseBody::Text(json_str),
                })
            }
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
