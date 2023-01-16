use crate::{HTTPMethod, RouteOrWSRoute, Router};

pub struct RouterRoute<'a> {
    pub router: &'a mut Router,
}

impl<'a> RouterRoute<'a> {
    pub fn route(&mut self, route: RouteOrWSRoute) -> &mut Self {
        match route {
            RouteOrWSRoute::Common(route) => match route.method {
                HTTPMethod::GET => {
                    let handler_id = format!("GET{}", route.path);
                    self.router.get.insert(route.path, handler_id.clone());
                    self.router.handler.insert(handler_id, route.handler);
                }
                HTTPMethod::POST => {
                    let handler_id = format!("POST{}", route.path);
                    self.router.post.insert(route.path, handler_id.clone());
                    self.router.handler.insert(handler_id, route.handler);
                }
            },
            RouteOrWSRoute::WS(route) => {
                let handler_id = format!("GET{}", route.path);
                self.router.get.insert(route.path, handler_id.clone());
                self.router.ws_handler.insert(handler_id, route.handler);
            }
        }

        self
    }
}
