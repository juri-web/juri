use super::Context;

pub type Route = (String, fn(context: Context));
#[derive(Clone)]
pub struct Router {
    pub get: Vec<Route>,
    pub post: Vec<Route>,
}

pub fn handle_router(context: &Context, router: Router) -> Option<Route> {
    let mut routes = Vec::<Route>::new();
    if context.method == "GET" {
        routes = router.get;
    } else if context.method == "POST" {
        routes = router.post;
    }
    for route in routes {
        if match_router_path(route.0.clone(), context.path.clone()) {
            return Some(route);
        }
    }
    None
}


fn match_router_path(route_path: String, path: String) -> bool {
    return route_path == path;
}