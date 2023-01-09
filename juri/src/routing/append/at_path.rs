use crate::{response::HTTPHandler, Router};
use std::sync::Arc;

pub struct AtPath<'a> {
    pub router: &'a mut Router,
    pub path: String,
}

impl<'a> AtPath<'a> {
    pub fn get(&mut self, handler: impl HTTPHandler + 'static) -> &mut Self {
        let handler_id = format!("GET{}", self.path);
        self.router
            .get
            .insert(self.path.clone(), handler_id.clone());
        self.router.handler.insert(handler_id, Arc::new(handler));

        self
    }

    pub fn post(&mut self, handler: impl HTTPHandler + 'static) -> &mut Self {
        let handler_id = format!("POST{}", self.path);
        self.router
            .post
            .insert(self.path.clone(), handler_id.clone());
        self.router.handler.insert(handler_id, Arc::new(handler));

        self
    }
}
