use crate::{Request, Response};

pub trait JuriPlugin: Send + Sync + 'static {
    fn request(&self, request: &mut Request) -> Option<Response>;
    fn response(&self, request: &Request, response: &mut Response);
}
