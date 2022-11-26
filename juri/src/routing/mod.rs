use crate::{request::HTTPMethod, Request, Response};
mod conversion_router_mod;
mod match_router_mod;
pub use conversion_router_mod::conversion_router;
pub use match_router_mod::{match_route, match_route_path, MatchRoute, MatchRouter};

type HandleFn = fn(request: &Request) -> crate::Result<Response>;
pub type Route = (HTTPMethod, String, HandleFn);

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

    #[deprecated(since = "0.5.0", note = "Please use the at function instead")]
    pub fn get(&mut self, path: &str, handle: HandleFn) {
        self.get.push((HTTPMethod::GET, path.to_string(), handle));
    }

    #[deprecated(since = "0.5.0", note = "Please use the at function instead")]
    pub fn post(&mut self, path: &str, handle: HandleFn) {
        self.post.push((HTTPMethod::POST, path.to_string(), handle));
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
        match route.0 {
            HTTPMethod::GET => self.router.get.push(route),
            HTTPMethod::POST => self.router.post.push(route),
        }
        self
    }
}

pub struct RouterAtPath<'a> {
    router: &'a mut Router,
    path: String,
}

impl<'a> RouterAtPath<'a> {
    pub fn get(&mut self, handle: HandleFn) -> &mut Self {
        self.router
            .get
            .push((HTTPMethod::GET, self.path.clone(), handle));
        self
    }

    pub fn post(&mut self, handle: HandleFn) -> &mut Self {
        self.router
            .post
            .push((HTTPMethod::POST, self.path.clone(), handle));
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
