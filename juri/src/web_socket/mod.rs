mod config;
mod frame;
mod message;
mod request;
mod response;
mod stream;

use crate::Request;
use async_trait::async_trait;
pub use config::WSConfig;
pub use message::Message;
pub use request::WSRequestExt;
pub use response::WSResponse;
pub use stream::WSStream;

#[async_trait]
pub trait WSHandler: Send + Sync {
    async fn call(&self, request: &Request) -> crate::Result<WSResponse>;
}

#[cfg(test)]
mod test {
    use super::{message::Message, WSRequestExt, WSResponse};
    use crate::Request;

    fn test_handle_success(request: &Request) -> crate::Result<WSResponse> {
        let mut ws = request.upgrader()?;

        println!("upgrader success");

        ws.on(|mut stream| async move {
            loop {
                let message = stream.read().await.unwrap();
                match message {
                    Message::Text(_) => todo!(),
                    Message::Binary(_) => todo!(),
                    Message::Ping(_) => todo!(),
                    Message::Pong(_) => todo!(),
                    Message::Close => {
                        return;
                    }
                }
            }
        });

        Ok(ws)
    }

    #[test]
    fn test_success() {
        let mut request = Request::default();

        request.protocol_and_version = "HTTP/1.1".to_string();

        request.headers.insert("Connection", "Upgrade");
        request.headers.insert("Upgrade", "websocket");
        request.headers.insert("Sec-WebSocket-Version", "13");
        request.headers.insert("Sec-WebSocket-Key", "-----");

        let _ws_response = test_handle_success(&request).unwrap();
    }
}
