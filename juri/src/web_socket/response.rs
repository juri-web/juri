use crate::Response;

use super::stream::WSStream;
use futures_util::{future::BoxFuture, FutureExt};
use sha1::{Digest, Sha1};
use std::{collections::HashMap, future::Future};

type BoxWebSocketHandler =
    Box<dyn FnOnce(WSStream) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub struct WSResponse {
    request_header_map: HashMap<String, String>,
    pub response: Response,
    pub callback: Option<BoxWebSocketHandler>,
}

impl WSResponse {
    pub fn new(response: Response) -> Self {
        WSResponse {
            request_header_map: HashMap::new(),
            response,
            callback: None,
        }
    }

    pub fn success(request_header_map: HashMap<String, String>) -> Self {
        let response = Response {
            status_code: 101,
            headers: HashMap::new(),
            body: crate::ResponseBody::None,
        };
        WSResponse {
            request_header_map,
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

    fn get_sec_websocket_accept(mut key: String) -> String {
        key.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let mut hasher = Sha1::new();
        hasher.update(key);

        let result = hasher.finalize();

        base64::encode(result)
    }

    pub fn into_response(&mut self) -> Response {
        self.response
            .headers
            .insert("Connection".to_string(), "Upgrade".to_string());
        self.response
            .headers
            .insert("Upgrade".to_string(), "websocket".to_string());

        let sec_websocket_accept = WSResponse::get_sec_websocket_accept(
            self.request_header_map
                .get("Sec-WebSocket-Key")
                .unwrap()
                .to_string(),
        );
        self.response
            .headers
            .insert("Sec-WebSocket-Accept".to_string(), sec_websocket_accept);
        self.response.clone()
    }
}
