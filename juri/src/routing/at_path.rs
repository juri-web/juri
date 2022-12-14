use crate::{response::HTTPHandler, HTTPMethod, Route, Router};
use std::sync::Arc;

pub struct RouterAtPath<'a> {
    pub router: &'a mut Router,
    pub path: String,
}

impl<'a> RouterAtPath<'a> {
    pub fn get(&mut self, handler: impl HTTPHandler + 'static) -> &mut Self {
        self.router.get.push(Route {
            method: HTTPMethod::GET,
            path: self.path.to_string(),
            handler: Arc::new(handler),
        });
        self
    }

    pub fn post(&mut self, handler: impl HTTPHandler + 'static) -> &mut Self {
        self.router.post.push(Route {
            method: HTTPMethod::POST,
            path: self.path.to_string(),
            handler: Arc::new(handler),
        });
        self
    }
}
