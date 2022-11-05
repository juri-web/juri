use juri::{json::ResponseExt, Request, Response};
use serde_derive::{Deserialize, Serialize};
use std::fs;

pub static TEMPLATE_PATH: &str = "./examples/template";

// 首页
pub fn handle_index(request: Request) -> juri::Result<Response> {
    println!(
        "query a={}",
        request
            .query("a")
            .map_or("".to_string(), |q| q.as_str().to_string())
    );
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/hello.html")).unwrap();
    Ok(Response::html_str(&content))
}

#[derive(Deserialize, Serialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn handle_params(request: Request) -> juri::Result<Response> {
    let point = Point { x: 2, y: 3 };
    println!(
        "param bb={}",
        request
            .param("bb")
            .map_or("".to_string(), |q| q.as_str().to_string())
    );
    Ok(Response::json(&point))
}
