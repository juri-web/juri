use juri::{Request, Response};
use std::fs;

pub static TEMPLATE_PATH: &str = "./examples/template";

pub fn upload_file(_request: Request) -> Response {
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/upload_file.html")).unwrap();
    println!("{:#?}", _request.header_map);
    Response::html_str(&content)
}

pub fn post_upload_file(_request: Request) -> Response {
    println!("{:#?}", _request.header_map);
    Response::json_str("{}")
}