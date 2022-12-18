
use juri::{get, Router, StaticFilePlugin, Response, WSResponse, WSRequest};
use std::{env, fs, net::SocketAddr};
pub static TEMPLATE_PATH: &str = "./web-scoket/template";

#[get("/ws", ws)]
pub fn handle_ws(request: &juri::Request) -> juri::Result<WSResponse> {
    let ws = request.upgrader().unwrap();
    Ok(ws)
}

#[get("/")]
pub fn handle_index(_request: &juri::Request) -> juri::Result<Response> {
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/index.html")).unwrap();
    Ok(Response::html_str(&content))
}

#[juri::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut router = Router::new();
    router.route(handle_index());
    router.route(handle_ws());

    let current_dir = env::current_dir().unwrap();
    let static_file_plugin = StaticFilePlugin::new(
        vec!["/static".to_string()],
        vec![current_dir.join("web-scoket").join("static")],
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr)
        .plugin(static_file_plugin)
        .server(router)
        .await?;
    Ok(())
}
