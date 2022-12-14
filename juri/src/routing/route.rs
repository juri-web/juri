
use crate::{request::HTTPMethod, response::HTTPHandler};
use std::rc::Rc;

#[derive(Clone)]
pub struct Route {
    pub method: HTTPMethod,
    pub path: String,
    pub handler: Rc<dyn HTTPHandler + 'static>,
}
