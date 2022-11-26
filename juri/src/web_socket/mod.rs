use crate::{response::IntoResponse, HTTPMethod, Request};
mod stream;
use std::future::Future;
use stream::WebSocketStream;

pub struct WebSocket<T> {
    callback: Option<T>,
}

impl<T, Fut> WebSocket<T>
where
    T: FnOnce(WebSocketStream) -> Fut + Send + Sync + 'static,
    Fut: Future + Send + 'static,
{
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

impl<T, Fut> WebSocket<T>
where
    T: FnOnce(WebSocketStream) -> Fut + Send + Sync + 'static,
    Fut: Future + Send + 'static,
{
    pub fn on(&mut self, callback: T) {
        self.callback = Some(callback);
    }
}

impl<T, Fut> IntoResponse for WebSocket<T>
where
    T: FnOnce(WebSocketStream) -> Fut + Send + Sync + 'static,
    Fut: Future + Send + 'static,
{
    fn into_response(self) -> crate::Response {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::WebSocket;
    use crate::{response::IntoResponse, Request, Response};

    fn test_handle_success(request: &Request) -> crate::Result<Response> {
        let mut ws = WebSocket::upgrader(&request).unwrap();

        ws.on(|_stream| async {});

        Ok(ws.into_response())
    }

    #[test]
    fn test_success() {
        let request = Request::new();
        let _ = test_handle_success(&request);
    }
}
