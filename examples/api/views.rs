use juri::{Request, Response};
use serde_derive::{Deserialize, Serialize};
use std::fs;

pub static TEMPLATE_PATH: &str = "./examples/template";
// 首页
pub fn handle_index(request: Request) -> Response {
    println!(
        "query a={}",
        request
            .query("a")
            .map_or("".to_string(), |q| q.as_str().to_string())
    );
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/hello.html")).unwrap();
    Response::html_str(&content)
}

#[derive(Deserialize, Serialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn handle_params(request: Request) -> Response {
    let point = Point { x: 2, y: 3 };
    let content = serde_json::to_string(&point).unwrap();
    Response::json_str(&content)
}
