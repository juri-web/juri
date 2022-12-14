use crate::{response::HTTPHandler, HTTPMethod, Request, Response, Route, Router};
use async_trait::async_trait;
use std::rc::Rc;

type Handler = fn(request: &Request) -> crate::Result<Response>;

pub struct RouterAtPath<'a> {
    pub router: &'a mut Router,
    pub path: String,
}

impl<'a> RouterAtPath<'a> {
    pub fn get(&mut self, handler: Handler) -> &mut Self {
        self.router.get.push(Route {
            method: HTTPMethod::GET,
            path: self.path.to_string(),
            handler: Rc::new(RouterAtPathHandler { handler }),
        });
        self
    }

    pub fn post(&mut self, handler: Handler) -> &mut Self {
        self.router.post.push(Route {
            method: HTTPMethod::POST,
            path: self.path.to_string(),
            handler: Rc::new(RouterAtPathHandler { handler }),
        });
        self
    }
}

struct RouterAtPathHandler {
    handler: Handler,
}

#[async_trait]
impl HTTPHandler for RouterAtPathHandler {
    async fn call(&self, request: &Request) -> crate::Result<Response> {
        (self.handler)(request)
    }
}
