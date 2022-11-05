mod api;
use juri::Router;
use std::net::SocketAddr;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut router = Router::new();

    router.get("/", api::views::handle_index);
    router.get("/aa/bb", api::views::handle_index);
    router.get("/aa/:bb", api::views::handle_params);
    router.get("/aa/:bb/cc", api::views::handle_params);
    router.get("/aa/:bb/:cc", api::views::handle_params);

    router.get("/mode", api::try_mode::handle_result_mode);
    router.get("/mode/error", api::error::handle_error_mode);

    router.get("/upload/file", api::upload::upload_file);
    router.post("/upload/file2", api::upload::post_upload_file);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr).server(router).await?;
    Ok(())
}
