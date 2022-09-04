mod api;
use juri::Juri;

fn main() {
    let mut router = Juri::new();
    router.get("/", api::views::handle_index);
    router.get("/aa/bb", api::views::handle_index);
    router.get("/aa/:bb", api::views::handle_params);
    router.get("/aa/:bb/cc", api::views::handle_params);
    router.get("/aa/:bb/:cc", api::views::handle_params);

    router.get("/upload/file", api::views::upload_file);
    router.post("/upload/file2", api::views::post_upload_file);
    router.run("127.0.0.1:7878");
}
