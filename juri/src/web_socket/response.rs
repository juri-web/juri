use crate::{http::Headers, Response};

use super::stream::WSStream;
use futures_util::{future::BoxFuture, FutureExt};
use sha1::{Digest, Sha1};
use std::future::Future;

type BoxWebSocketHandler =
    Box<dyn FnOnce(WSStream) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub struct WSResponse {
    request_headers: Headers,
    pub response: Response,
    pub callback: Option<BoxWebSocketHandler>,
}

impl WSResponse {
    pub fn new(response: Response) -> Self {
        WSResponse {
            request_headers: Headers::default(),
            response,
            callback: None,
        }
    }

    pub fn success(request_headers: Headers) -> Self {
        let response = Response {
            status_code: 101,
            headers: Default::default(),
            body: crate::ResponseBody::None,
        };
        WSResponse {
            response,
            callback: None,
            request_headers,
        }
    }

    pub fn on<F, Fut>(&mut self, callback: F)
    where
        F: FnOnce(WSStream) -> Fut + Send + Sync + 'static,
        Fut: Future + Send + 'static,
    {
        self.callback = Some(Box::new(|stream| (callback)(stream).map(|_| ()).boxed()));
    }

    fn get_sec_websocket_accept(mut key: String) -> String {
        key.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let mut hasher = Sha1::new();
        hasher.update(key);

        let result = hasher.finalize();

        base64::encode(result)
    }

    pub fn into_response(&mut self) -> Response {
        self.response.headers.insert("Connection", "Upgrade");
        self.response.headers.insert("Upgrade", "websocket");

        let sec_websocket_accept = WSResponse::get_sec_websocket_accept(
            self.request_headers
                .get(&"Sec-WebSocket-Key".to_lowercase())
                .unwrap()
                .last()
                .unwrap()
                .to_string(),
        );
        self.response
            .headers
            .insert("Sec-WebSocket-Accept", &sec_websocket_accept);
        self.response.clone()
    }
}
