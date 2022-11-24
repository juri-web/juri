use juri::Router;
mod views;

pub fn router() -> Router{
    let mut router = Router::new();
    router.root("/user");

    router.route(views::handle_index());
    router
}