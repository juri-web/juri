use crate::{request::HTTPMethod, response::HTTPHandler};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct Route {
    pub method: HTTPMethod,
    pub path: String,
    pub handler: Arc<dyn HTTPHandler + 'static>,
}

pub type RouteMap = HashMap<String, String>;
pub type RouteHandlerMap = HashMap<String, Arc<dyn HTTPHandler + 'static>>;