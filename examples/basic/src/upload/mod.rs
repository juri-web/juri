use juri::{handler, json::JsonResponseExt, Request, Response, Router};
use std::fs;

pub static TEMPLATE_PATH: &str = "./basic/template";

#[handler]
pub fn upload_file(_request: &Request) -> juri::Result<Response> {
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/upload_file.html")).unwrap();
    Ok(Response::html(&content))
}

#[handler]
pub fn post_upload_file(_request: &Request) -> juri::Result<Response> {
    Ok(Response::json("{}")?)
}

pub fn router() -> Router {
    let mut router = Router::new();
    router.root("/upload");

    router.at("/file").get(upload_file);
    router.at("/file2").post(post_upload_file);
    router
}
