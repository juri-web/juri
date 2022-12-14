use crate::{request::HTTPMethod, response::HTTPHandler};
use std::sync::Arc;

#[derive(Clone)]
pub struct Route {
    pub method: HTTPMethod,
    pub path: String,
    pub handler: Arc<dyn HTTPHandler + 'static>,
}
