use super::{
    super::{RouteHandlerMap, WSRouterHandlerMap},
    route::MatchRoute,
};
use crate::{request::HTTPMethod, response::HTTPHandler, web_socket::WSHandler, Request, Router};
use std::{cmp::Ordering, sync::Arc};

#[derive(Default)]
pub struct MatchRouter {
    pub get: Vec<MatchRoute>,
    pub post: Vec<MatchRoute>,

    handler: RouteHandlerMap,
    ws_handler: WSRouterHandlerMap,
}

impl MatchRouter {
    pub fn new(router: Router) -> Self {
        let mut match_router = MatchRouter::default();
        match_router.summary_router(&router, String::default());
        match_router.sort();
        match_router
    }
    pub fn summary_router(&mut self, router: &Router, mut root: String) {
        if let Some(root_path) = &router.root {
            root.push_str(root_path.as_str());
        }

        self.get.append(
            &mut router
                .get
                .iter()
                .map(|route| {
                    MatchRoute::new(format!("{}{}", root, route.0.clone()), route.1.clone())
                })
                .collect(),
        );
        self.post.append(
            &mut router
                .post
                .iter()
                .map(|route| {
                    MatchRoute::new(format!("{}{}", root, route.0.clone()), route.1.clone())
                })
                .collect(),
        );

        self.handler.extend(router.handler.clone());
        self.ws_handler.extend(router.ws_handler.clone());

        for router in router.router.iter() {
            MatchRouter::summary_router(self, router, root.clone());
        }
    }

    pub fn sort(&mut self) {
        self.get.sort_by(|a, b| {
            if a.params.is_empty() && !b.params.is_empty() {
                Ordering::Less
            } else if !a.params.is_empty() && b.params.is_empty() {
                Ordering::Greater
            } else {
                b.path.cmp(&a.path)
            }
        });
        self.post.sort_by(|a, b| {
            if a.params.is_empty() && !b.params.is_empty() {
                Ordering::Less
            } else if !a.params.is_empty() && b.params.is_empty() {
                Ordering::Greater
            } else {
                b.path.cmp(&a.path)
            }
        });
    }
}

pub enum MatchRouteHandler {
    Common(Arc<dyn HTTPHandler>),
    WS(Arc<dyn WSHandler>),
    None,
}

impl MatchRouter {
    pub fn match_handler(&self, request: &mut Request) -> MatchRouteHandler {
        match request.method {
            HTTPMethod::GET => {
                for route in self.get.iter() {
                    if let Some(map) = route.match_params(request.path.clone()) {
                        request.params_map = map;
                        return match self.handler.get(&route.handler) {
                            Some(handler) => MatchRouteHandler::Common(handler.clone()),
                            None => match self.ws_handler.get(&route.handler) {
                                Some(handler) => MatchRouteHandler::WS(handler.clone()),
                                None => MatchRouteHandler::None,
                            },
                        };
                    }
                }
            }
            HTTPMethod::POST => {
                for route in self.post.iter() {
                    if let Some(map) = route.match_params(request.path.clone()) {
                        request.params_map = map;
                        return match self.handler.get(&route.handler) {
                            Some(handler) => MatchRouteHandler::Common(handler.clone()),
                            None => MatchRouteHandler::None,
                        };
                    }
                }
            }
        };

        MatchRouteHandler::None
    }
}

#[cfg(test)]
mod test {
    use super::MatchRouter;
    use crate::prelude::*;

    #[get("/hi", internal)]
    fn hi(_request: &Request) -> crate::Result<Response> {
        Ok(Response::html("hi"))
    }

    fn child_child_router() -> Router {
        let mut router = Router::default();
        router.root("/child");
        router.route(hi());
        router
    }

    fn child_router() -> Router {
        let mut router = Router::default();
        router.root("/me");
        router.route(hi());
        router.router(child_child_router());
        router
    }

    #[test]
    fn test_match_router() {
        let mut router = Router::default();
        router.root("/my");
        router.route(hi());
        router.router(child_router());

        let match_router = MatchRouter::new(router);
        let get_list = match_router.get;

        assert_eq!(get_list[0].path, String::from("^/my/me/hi$"));
        assert_eq!(get_list[1].path, String::from("^/my/me/child/hi$"));
        assert_eq!(get_list[2].path, String::from("^/my/hi$"));
    }
}
