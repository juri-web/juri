mod api;
use juri::Juri;

fn main() {
    let mut router = Juri::new();
    router.get("/", api::views::handle_index);
    router.get("/aa/bb", api::views::handle_index);
    router.get("/aa/:bb", api::views::handle_params);
    router.get("/aa/:bb/cc", api::views::handle_params);
    router.get("/aa/:bb/:cc", api::views::handle_params);

    router.get_result_mode("/mode", api::try_mode::handle_result_mode);
    router.get_error_mode("/mode/error", api::error::handle_error_mode);

    router.get("/upload/file", api::views::upload_file);
    router.post("/upload/file2", api::views::post_upload_file);
    router.run("127.0.0.1:7878");
}
