use crate::{response::IntoResponse, HTTPMethod, Request};
mod stream;
use futures_util::{future::BoxFuture, FutureExt};
use std::future::Future;
use stream::WebSocketStream;

type BoxWebSocketHandler =
    Box<dyn FnOnce(WebSocketStream) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub struct WebSocket {
    callback: Option<BoxWebSocketHandler>,
}

impl WebSocket {
    pub fn upgrader(request: &Request) -> crate::Result<Self> {
        if request.method != HTTPMethod::GET {
            Err(crate::Error {
                code: 405,
                reason: "Method Not Allowed".to_string(),
            })?
        }

        if request.protocol_and_version != "HTTP/1.1" {
            Err(crate::Error {
                code: 406,
                reason: "Not Acceptable".to_owned(),
            })?;
        }

        if let Some(connection) = request.header("Connection") {
            if connection == "Upgrade" {
                if let Some(upgrade) = request.header("Upgrade") {
                    if upgrade == "websocket" {
                        return Ok(WebSocket { callback: None });
                    }
                }
            }
        }
        Err(crate::Error {
            code: 406,
            reason: "Not Acceptable".to_owned(),
        })?
    }
}

impl WebSocket {
    pub fn on<F, Fut>(&mut self, callback: F)
    where
        F: FnOnce(WebSocketStream) -> Fut + Send + Sync + 'static,
        Fut: Future + Send + 'static,
    {
        self.callback = Some(Box::new(|stream| (callback)(stream).map(|_| ()).boxed()));
    }
}

impl IntoResponse for WebSocket {
    fn into_response(self) -> crate::Response {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::WebSocket;
    use crate::{response::IntoResponse, Request};
    use std::any::{Any, TypeId};

    fn test_handle_success(request: &Request) -> crate::Result<impl IntoResponse> {
        let mut ws = WebSocket::upgrader(&request).unwrap();

        ws.on(|_stream| async {});

        Ok(ws)
    }

    #[test]
    fn test_success_type_id() {
        let mut request = Request::new();

        request.protocol_and_version = "HTTP/1.1".to_string();

        request
            .header_map
            .insert("Connection".to_string(), "Upgrade".to_string());
        request
            .header_map
            .insert("Upgrade".to_string(), "websocket".to_string());

        let into = test_handle_success(&request).unwrap();
        println!(
            "{:?} {:?} {:?}",
            TypeId::of::<WebSocket>(),
            into.type_id(),
            TypeId::of::<dyn IntoResponse>()
        );
    }

    #[test]
    fn test_success_type() {
        let mut request = Request::new();

        request.protocol_and_version = "HTTP/1.1".to_string();

        request
            .header_map
            .insert("Connection".to_string(), "Upgrade".to_string());
        request
            .header_map
            .insert("Upgrade".to_string(), "websocket".to_string());

        let into = test_handle_success(&request).unwrap();

        if let Some(_ws) = <dyn Any>::downcast_ref::<WebSocket>(&into) {
            println!("is WebSocket");
        } else {
            println!("not WebSocket");
        }
    }
}
