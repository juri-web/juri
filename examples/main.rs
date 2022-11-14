mod api;
use juri::{Router, StaticFilePlugin};
use std::{env, net::SocketAddr};

fn init_router() -> Router {
    let mut router = Router::new();

    router.get("/", api::views::handle_index);
    router.get("/aa/bb", api::views::handle_index);
    router.get("/aa/:bb", api::views::handle_params);
    router.get("/aa/:bb/cc", api::views::handle_params);
    router.get("/aa/:bb/:cc", api::views::handle_params);

    router.get("/mode", api::try_mode::handle_result_mode);

    router.get("/upload/file", api::upload::upload_file);
    router.post("/upload/file2", api::upload::post_upload_file);

    router.get("/file/static", api::views::handle_static_file);

    router.post("/json/request", api::json::handle_request_json);
    router.post("/json/response", api::json::handle_response_json);

    router
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let router = init_router();

    let current_dir = env::current_dir().unwrap();
    let static_file_plugin = StaticFilePlugin::new(
        vec!["/static".to_string()],
        vec![current_dir.join("examples").join("static")],
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr)
        .plugin(static_file_plugin)
        .server(router)
        .await?;
    Ok(())
}
