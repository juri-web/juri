mod at_path;
mod conversion_router;
mod match_router;
mod route;

use crate::request::HTTPMethod;
pub use at_path::RouterAtPath;
pub use conversion_router::conversion_router;
pub use match_router::{match_route, match_route_path, MatchRoute, MatchRouter};
pub use route::Route;

pub struct Router {
    root: Option<String>,
    get: Vec<Route>,
    post: Vec<Route>,
    router: Vec<Router>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            root: None,
            get: vec![],
            post: vec![],
            router: vec![],
        }
    }

    pub fn root(&mut self, root: &str) {
        self.root = Some(root.to_string());
    }

    pub fn at<'a>(&'a mut self, path: &str) -> RouterAtPath<'a> {
        RouterAtPath {
            router: self,
            path: path.to_string(),
        }
    }

    pub fn route(&mut self, route: Route) -> RouterRoute {
        let mut router_route = RouterRoute { router: self };
        router_route.route(route);
        router_route
    }

    pub fn router(&mut self, router: Router) {
        self.router.push(router);
    }
}

pub struct RouterRoute<'a> {
    router: &'a mut Router,
}

impl<'a> RouterRoute<'a> {
    pub fn route(&mut self, route: Route) -> &mut Self {
        match route.method {
            HTTPMethod::GET => self.router.get.push(route),
            HTTPMethod::POST => self.router.post.push(route),
        }
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{Request, Response, Router, handler};

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
