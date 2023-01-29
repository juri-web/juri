use juri::{handler, json::JsonResponseExt, Request, Response, Router};
use std::{fs, path::Path};

pub static TEMPLATE_PATH: &str = "./basic/template";
pub static UPLOAD_PATH: &str = "./basic/upload";

#[handler]
pub fn upload_file(_request: &Request) -> juri::Result<Response> {
    let content = fs::read_to_string(TEMPLATE_PATH.to_owned() + "/upload_file.html").unwrap();
    Ok(Response::html(&content))
}

#[handler]
pub fn post_upload_file(request: &Request) -> juri::Result<Response> {
    let file = request.file("file").unwrap();
    let file_name = file.file_name.clone().unwrap();
    let path = format!("{UPLOAD_PATH}/{file_name}");
    let path = Path::new(&path);
    file.copy(path).unwrap();
    Ok(Response::json("{}")?)
}

pub fn router() -> Router {
    let mut router = Router::new();
    router.root("/upload");

    router.at("/file").get(upload_file);
    router.at("/file2").post(post_upload_file);
    router
}
