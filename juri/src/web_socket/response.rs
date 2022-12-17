use crate::Response;

use super::stream::WSStream;
use futures_util::{future::BoxFuture, FutureExt};
use std::{collections::HashMap, future::Future};

type BoxWebSocketHandler =
    Box<dyn FnOnce(WSStream) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub struct WSResponse {
    response: Response,
    callback: Option<BoxWebSocketHandler>,
}

impl WSResponse {
    pub fn new(response: Response) -> Self {
        WSResponse {
            response,
            callback: None,
        }
    }

    pub fn success() -> Self {
        let response = Response {
            status_code: 200,
            headers: HashMap::new(),
            body: crate::ResponseBody::None,
        };
        WSResponse {
            response,
            callback: None,
        }
    }

    pub fn on<F, Fut>(&mut self, callback: F)
    where
        F: FnOnce(WSStream) -> Fut + Send + Sync + 'static,
        Fut: Future + Send + 'static,
    {
        self.callback = Some(Box::new(|stream| (callback)(stream).map(|_| ()).boxed()));
    }

    pub fn into_response(self) -> Response {
        self.response
    }
}
