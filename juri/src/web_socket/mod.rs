mod request;
mod response;
mod stream;

use crate::Request;
use async_trait::async_trait;
pub use request::WSRequest;
pub use response::WSResponse;

#[async_trait]
pub trait WSHandler {
    async fn call<WS>(&self, request: &Request) -> crate::Result<WSResponse>
    where
        WS: WSRequest;
}

#[cfg(test)]
mod test {
    use super::{WSRequest, WSResponse};
    use crate::Request;

    fn test_handle_success(request: &Request) -> crate::Result<WSResponse> {
        let mut ws = request.upgrader().unwrap();

        ws.on(|_stream| async {
            loop {
                
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
