pub mod views;
pub mod try_mode;
pub mod upload;
pub mod json;
use juri::get;

#[get("/index")]
pub fn handle_index(_request: &juri::Request) -> juri::Result<juri::Response> {
    Ok(juri::Response::html_str("Hello Juri"))
}