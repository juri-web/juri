mod api;
use juri::Juri;

fn main() {
    let mut router = Juri::new();
    router.get("/", api::views::handle_index);
    router.get("/aa/:bb", api::views::handle_params);
    router.get("/aa/:bb/cc", api::views::handle_params);
    router.get("/aa/:bb/:cc", api::views::handle_params);
    router.run("127.0.0.1:7878");
}
