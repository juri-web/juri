use super::Response;
use crate::Request;
use async_trait::async_trait;

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

#[async_trait]
pub trait HTTPHandler {
    async fn call(&self, request: &Request) -> crate::Result<Response>;
}
