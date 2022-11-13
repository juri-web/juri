use crate::{Request, Response};
mod static_file;

pub use static_file::StaticFilePlugin;

pub trait JuriPlugin: Send + Sync + 'static {
    fn request(&self, request: &mut Request) -> Option<Response>;
    fn response(&self, request: &Request, response: &mut Response);
}
