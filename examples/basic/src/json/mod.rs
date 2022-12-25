use juri::{
    handler,
    json::{JsonRequestExt, JsonResponseExt},
    Request, Response, Router,
};
use serde_derive::{Deserialize, Serialize};

#[handler]
pub fn handle_request(request: &Request) -> juri::Result<Response> {
    let body_json = request.json_value()?;
    let organization_id = body_json["organizationId"].as_str().map_or("", |v| v);
    Ok(Response::html(organization_id))
}

#[derive(Deserialize, Serialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[handler]
pub fn handle_response(_request: &Request) -> juri::Result<Response> {
    let point = Point { x: 2, y: 3 };
    Ok(Response::json(&point)?)
}

pub fn router() -> Router {
    let mut router = Router::new();
    router.root("/json");

    router.at("/request").post(handle_request);
    router.at("/response").post(handle_response);

    router
}
