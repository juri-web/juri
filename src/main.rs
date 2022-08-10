mod api;
mod lib;
mod setting;

use lib::main::Junior;

fn main() {
    let mut router = Junior::new();
    router.get("/", api::views::handle_index);
    router.get("", api::views::handle_404);
    router.run(setting::ADDRESS);
}
