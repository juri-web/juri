mod json;
mod route;
mod upload;
mod url;
mod views;

use juri::{plugin::StaticFilePlugin, Router};
use std::{collections::HashMap, env, net::SocketAddr};

fn init_router() -> Router {
    let mut router = Router::new();

    router.at("/").get(views::handle_index);
    router.at("/file/static").get(views::handle_static_file);

    router.router(upload::router());
    router.router(json::router());
    router.router(route::router());
    router.router(url::router());

    router
}

#[juri::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let router = init_router();

    let current_dir = env::current_dir().unwrap();
    let static_file_plugin = StaticFilePlugin::new(HashMap::from([(
        "/static",
        vec![current_dir.join("example").join("static")],
    )]));

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr)
        .plugin(static_file_plugin)
        .server(router)
        .await?;
    Ok(())
}
