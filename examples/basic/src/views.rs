use std::path::PathBuf;
use juri::{handler, Request, Response};
use std::fs;

pub static TEMPLATE_PATH: &str = "./basic/template";

// 首页
#[handler]
pub fn handle_index(request: &Request) -> juri::Result<Response> {
    println!(
        "query a={}",
        request
            .query("a")
            .map_or("".to_string(), |q| q.as_str().to_string())
    );
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/hello.html")).unwrap();
    Ok(Response::html(&content))
}

#[handler]
pub fn handle_static_file(request: &Request) -> juri::Result<Response> {
    println!("fill path={}", request.full_path);
    // let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/static_file.html")).unwrap();
    // Ok(Response::html(&content))

    let path = PathBuf::from(&(TEMPLATE_PATH.to_owned() + "/static_file.html"));
    Ok(Response::html_file(path)?)
}
