mod append;
mod match_router;
mod route;

use append::{AtPath, RouterRoute};
pub use match_router::{match_route_params, MatchRouteHandler, MatchRouter};
pub use route::{Route, RouteHandlerMap, RouteMap, RouteOrWSRoute, WSRoute, WSRouterHandlerMap};

#[derive(Default)]
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
        Router::default()
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
        Ok(Response::html("Hello Juri"))
    }

    #[test]
    fn test_at() {
        let mut router = Router::default();

        router.at("/").get(handle_index_at).post(handle_index_at);
    }
}
