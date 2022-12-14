use juri::get;
use juri::Response;

#[get("/login")]
pub fn handle_index(_request: &juri::Request) -> juri::Result<Response> {
    Ok(juri::Response::html_str("User Login"))
}