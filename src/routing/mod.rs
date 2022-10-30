mod http_method;
use crate::{Request, Response};

pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Router {}
    }

    pub fn route(
        &mut self,
        path: &str,
        handle: fn(request: Request) -> crate::Result<Response>,
    ) -> &mut Self {
        self
    }

    pub fn group() {}
}
