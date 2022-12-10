use crate::{response::HTTPHandler, HTTPMethod, IntoResponse, Request, Response, Route, Router};
use async_trait::async_trait;
use std::rc::Rc;

type Handler = fn(request: &Request) -> crate::Result<Response>;

pub struct AtPath<'a> {
    pub router: &'a mut Router,
    pub path: String,
}

impl<'a> AtPath<'a> {
    pub fn get(&mut self, handler: Handler) -> &mut Self {
        self.router.get.push(Route {
            method: HTTPMethod::GET,
            path: self.path.to_string(),
            handler: Rc::new(AtPathHandler { handler }),
        });
        self
    }

    pub fn post(&mut self, handler: Handler) -> &mut Self {
        self.router.post.push(Route {
            method: HTTPMethod::POST,
            path: self.path.to_string(),
            handler: Rc::new(AtPathHandler { handler }),
        });
        self
    }
}

struct AtPathHandler {
    handler: Handler,
}

#[async_trait]
impl HTTPHandler for AtPathHandler {
    async fn call(&self, request: &Request) -> crate::Result<Box<dyn IntoResponse>> {
        match (self.handler)(request) {
            Ok(v) => Ok(Box::new(v)),
            Err(e) => Err(e),
        }
    }
}
