use std::sync::Arc;

use super::Context;

pub type Route = (String, fn(context: Context));
#[derive(Clone)]
pub struct Router {
    pub get: Vec<Route>,
    pub post: Vec<Route>,
}

pub fn handle_router(context: &Context, router: Arc<Router>) -> Option<fn(Context)> {
    if context.method == "GET" {
        for route in &router.get {
            if match_router_path(route.0.clone(), context.path.clone()) {
                return Some(route.1);
            }
        }
    } else if context.method == "POST" {
        for route in &router.post {
            if match_router_path(route.0.clone(), context.path.clone()) {
                return Some(route.1);
            }
        }
    }
    None
}

fn match_router_path(route_path: String, path: String) -> bool {
    return route_path == path;
}
