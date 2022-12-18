use super::{
    super::{RouteHandlerMap, WSRouterHandlerMap},
    route::MatchRoute,
};
use crate::{request::HTTPMethod, response::HTTPHandler, Request, Router, WSHandler};
use std::{cmp::Ordering, collections::HashMap, sync::Arc};

pub struct MatchRouter {
    pub get: Vec<MatchRoute>,
    pub post: Vec<MatchRoute>,

    handler: RouteHandlerMap,
    ws_handler: WSRouterHandlerMap,
}

impl MatchRouter {
    pub fn new(router: Router) -> Self {
        let mut match_router = MatchRouter {
            get: vec![],
            post: vec![],
            handler: HashMap::new(),
            ws_handler: HashMap::new(),
        };
        match_router.summary_router(&router);
        match_router.sort();
        match_router
    }
    pub fn summary_router(&mut self, router: &Router) {
        if let Some(root_path) = &router.root {
            self.get.append(
                &mut router
                    .get
                    .iter()
                    .map(|route| {
                        MatchRoute::new(
                            format!("{}{}", root_path, route.0.clone()),
                            route.1.clone(),
                        )
                    })
                    .collect(),
            );
            self.post.append(
                &mut router
                    .post
                    .iter()
                    .map(|route| {
                        MatchRoute::new(
                            format!("{}{}", root_path, route.0.clone()),
                            route.1.clone(),
                        )
                    })
                    .collect(),
            );
        } else {
            self.get.append(
                &mut router
                    .get
                    .iter()
                    .map(|route| MatchRoute::new(route.0.clone(), route.1.clone()))
                    .collect(),
            );
            self.post.append(
                &mut router
                    .post
                    .iter()
                    .map(|route| MatchRoute::new(route.0.clone(), route.1.clone()))
                    .collect(),
            );
        }

        for router in router.router.iter() {
            MatchRouter::summary_router(self, router);
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
    COMMON(Arc<dyn HTTPHandler>),
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
                            Some(handler) => MatchRouteHandler::COMMON(handler.clone()),
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
                            Some(handler) => MatchRouteHandler::COMMON(handler.clone()),
                            None => MatchRouteHandler::None,
                        };
                    }
                }
            }
        };

        MatchRouteHandler::None
    }
}

// #[test]
// fn test_match_route_path() {
//     let params_map = match_route_path("^/aa$".to_string(), vec![], "/aa".to_string());
//     assert_ne!(params_map, None);

//     let params_map = match_route_path(
//         "^/aa/([^/]*?)$".to_string(),
//         vec!["bb".to_string()],
//         "/aa/11".to_string(),
//     );
//     assert_ne!(params_map, None);

//     let params_map = match_route_path(
//         "^/aa/([^/]*?)/cc$".to_string(),
//         vec!["bb".to_string()],
//         "/aa/11/cc".to_string(),
//     );
//     assert_ne!(params_map, None);

//     let params_map = match_route_path(
//         "^/aa/(.+)$".to_string(),
//         vec!["bb".to_string()],
//         "/aa/11/cc".to_string(),
//     );
//     assert_ne!(params_map, None);
// }
