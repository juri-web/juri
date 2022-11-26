pub mod json;
pub mod try_mode;
pub mod upload;
pub mod views;
use juri::{get, post};

#[get("/index")]
pub fn handle_index(_request: &juri::Request) -> juri::Result<juri::Response> {
    Ok(juri::Response::html_str("Hello Juri"))
}

#[post("/index")]
pub fn handle_index_post(_request: &juri::Request) -> juri::Result<juri::Response> {
    Ok(juri::Response::html_str("Hello Juri"))
}
