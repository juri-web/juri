use juri::{Router, StaticFilePlugin};
use std::{env, net::SocketAddr};
mod views;

#[juri::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut router = Router::new();
    router.route(views::handle_index());
    router.route(views::handle_ws());

    let current_dir = env::current_dir().unwrap();
    let static_file_plugin = StaticFilePlugin::new(
        vec!["/static".to_string()],
        vec![current_dir.join("web-socket").join("static")],
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr)
        .plugin(static_file_plugin)
        .server(router)
        .await?;
    Ok(())
}
