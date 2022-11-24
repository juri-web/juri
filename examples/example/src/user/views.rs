use juri::get;

#[get("/login")]
pub fn handle_index(_request: &juri::Request) -> juri::Result<juri::Response> {
    Ok(juri::Response::html_str("User Login"))
}