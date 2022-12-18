mod append;
mod match_router;
mod route;

use append::{AtPath, RouterRoute};
pub use match_router::{MatchRouteHandler, MatchRouter, match_route_params};
pub use route::{Route, RouteHandlerMap, RouteMap, RouteOrWSRoute, WSRoute, WSRouterHandlerMap};
use std::collections::HashMap;

pub struct Router {
    root: Option<String>,
    get: RouteMap,
    post: RouteMap,
    handler: RouteHandlerMap,
    ws_handler: WSRouterHandlerMap,
    router: Vec<Router>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            root: None,
            get: HashMap::new(),
            post: HashMap::new(),
            handler: HashMap::new(),
            ws_handler: HashMap::new(),

            router: vec![],
        }
    }

    pub fn root(&mut self, root: &str) {
        self.root = Some(root.to_string());
    }

    pub fn at<'a>(&'a mut self, path: &str) -> AtPath<'a> {
        AtPath {
            router: self,
            path: path.to_string(),
        }
    }

    pub fn route(&mut self, route: RouteOrWSRoute) -> RouterRoute {
        let mut router_route = RouterRoute { router: self };
        router_route.route(route);
        router_route
    }

    pub fn router(&mut self, router: Router) {
        self.router.push(router);
    }
}

#[cfg(test)]
mod test {
    use crate::{handler, Request, Response, Router};

    #[handler(internal)]
    pub async fn handle_index_at(_request: &Request) -> crate::Result<Response> {
        Ok(Response::html_str("Hello Juri"))
    }

    #[test]
    fn test_at() {
        let mut router = Router::new();

        router.at("/").get(handle_index_at).post(handle_index_at);
    }
}
