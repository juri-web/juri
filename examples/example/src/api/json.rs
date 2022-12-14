use juri::{
    json::{RequestExt, ResponseExt},
    Request, Response,
    handler
};
use serde_derive::{Deserialize, Serialize};

#[handler]
pub fn handle_request_json(request: &Request) -> juri::Result<Response> {
    let body_json = request.json_value().unwrap();
    let organization_id = body_json["organizationId"].as_str().map_or("", |v| v);
    Ok(Response::html_str(organization_id))
}

#[derive(Deserialize, Serialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[handler]
pub fn handle_response_json(_request: &Request) -> juri::Result<Response> {
    let point = Point { x: 2, y: 3 };
    Ok(Response::json(&point))
}
