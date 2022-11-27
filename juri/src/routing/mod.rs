use crate::{request::HTTPMethod, response::HTTPHandler};
use std::rc::Rc;
mod conversion_router_mod;
mod match_router_mod;
pub use conversion_router_mod::conversion_router;
pub use match_router_mod::{match_route, match_route_path, MatchRoute, MatchRouter};
mod at_path;
pub use at_path::AtPath;

#[derive(Clone)]
pub struct Route {
    method: HTTPMethod,
    path: String,
    handler: Rc<dyn HTTPHandler + 'static>,
}

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

    pub fn at<'a>(&'a mut self, path: &str) -> AtPath<'a> {
        AtPath {
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
    use crate::{Request, Response, Router};

    pub fn handle_index_at(_request: &Request) -> crate::Result<Response> {
        Ok(Response::html_str("Hello Juri"))
    }

    #[test]
    fn test_at() {
        let mut router = Router::new();

        router.at("/").get(handle_index_at).post(handle_index_at);
    }
}
