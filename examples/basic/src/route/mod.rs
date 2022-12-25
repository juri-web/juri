use juri::{get, post, Response, Router};

#[get("/login")]
pub fn handle(_request: &juri::Request) -> juri::Result<Response> {
    Ok(Response::html("Login"))
}

#[post("/login")]
pub fn handle_post(_request: &juri::Request) -> juri::Result<Response> {
    Ok(Response::html("Login Success"))
}

pub fn router() -> Router {
    let mut router = Router::new();
    router.root("/route");

    router.route(handle());
    router.route(handle_post());
    router
}
