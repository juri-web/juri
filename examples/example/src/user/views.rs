use juri::get;
use juri::IntoResponse;

#[get("/login")]
pub fn handle_index(_request: &juri::Request) -> juri::Result<impl IntoResponse> {
    Ok(juri::Response::html_str("User Login"))
}