mod frame;
mod message;
mod request;
mod response;
mod stream;

use crate::Request;
use async_trait::async_trait;
pub use request::WSRequest;
pub use response::WSResponse;

#[async_trait]
pub trait WSHandler {
    async fn call(&self, request: &Request) -> crate::Result<WSResponse>;
}

#[cfg(test)]
mod test {
    use super::{message::Message, WSRequest, WSResponse};
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
        let mut request = Request::new();

        request.protocol_and_version = "HTTP/1.1".to_string();

        request
            .header_map
            .insert("Connection".to_string(), "Upgrade".to_string());
        request
            .header_map
            .insert("Upgrade".to_string(), "websocket".to_string());

        let _ws_response = test_handle_success(&request).unwrap();
    }
}
