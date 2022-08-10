mod api;
use juri::Juri;

fn main() {
    let mut router = Juri::new();
    router.get("/", api::views::handle_index);
    router.get("", api::views::handle_404);
    router.run("127.0.0.1:7878");
}
