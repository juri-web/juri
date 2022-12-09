mod api;
mod user;
use juri::{Router, StaticFilePlugin};
use std::{env, net::SocketAddr};

fn init_router() -> Router {
    let mut router = Router::new();
    router
        .route(api::handle_index())
        .route(api::handle_index_post());
    router.at("/").get(api::views::handle_index);
    router.at("/aa/bb").get(api::views::handle_index);
    router.at("/aa/:bb").get(api::views::handle_params);
    router.at("/aa/:bb/cc").get(api::views::handle_params);
    router.at("/aa/:bb/:cc").get(api::views::handle_params);

    router.at("/mode").get(api::try_mode::handle_result_mode);

    router.at("/upload/file").get(api::upload::upload_file);
    router
        .at("/upload/file2")
        .post(api::upload::post_upload_file);

    router
        .at("/file/static")
        .get(api::views::handle_static_file);

    router
        .at("/json/request")
        .post(api::json::handle_request_json);
    router
        .at("/json/response")
        .post(api::json::handle_response_json);

    router.router(user::router());

    router
}

#[juri::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let router = init_router();

    let current_dir = env::current_dir().unwrap();
    let static_file_plugin = StaticFilePlugin::new(
        vec!["/static".to_string()],
        vec![current_dir.join("example").join("static")],
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr)
        .plugin(static_file_plugin)
        .server(router)
        .await?;
    Ok(())
}
