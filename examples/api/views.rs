use juri::{json::{ResponseExt}, Request, Response};
use serde_derive::{Deserialize, Serialize};
use std::fs;

pub static TEMPLATE_PATH: &str = "./examples/template";

pub fn upload_file(_request: Request) -> Response {
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/upload_file.html")).unwrap();
    Response::html_str(&content)
}

pub fn post_upload_file(request: Request) -> Response {
    let a = String::from_utf8(request.body_bytes.clone()).unwrap();
    println!("{:?}", a);

    request.form_data();
    Response::json_str("{}")
}


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
    println!(
        "param bb={}",
        request
            .param("bb")
            .map_or("".to_string(), |q| q.as_str().to_string())
    );
    Response::json(&point)
}
