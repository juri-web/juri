use juri::{handler, Request, Response, Router};

pub fn router() -> Router {
    let mut router = Router::new();
    router.root("/url");

    router.at("").get(handle);

    // TODO bug: /url/ 没有匹配成功，而是匹配 / 成功了
    // router.at("/").get(handle);

    router.at("/aa/bb").get(handle);
    router.at("/aa/:bb").get(handle);
    router.at("/aa/:bb/cc").get(handle);
    router.at("/aa/:bb/:cc").get(handle);

    router
}

#[handler]
fn handle(request: &Request) -> juri::Result<Response> {
    let full_path = request.path.clone();
    Ok(Response::html(&format!("hi {full_path}")))
}
