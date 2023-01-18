use crate::{HTTPHandler, HTTPMethod};
use std::{collections::HashMap, sync::Arc};
mod ws_route;
pub use ws_route::{WSRoute, WSRouterHandlerMap};

pub enum RouteOrWSRoute {
    Common(Route),
    WS(WSRoute),
}

#[derive(Clone)]
pub struct Route {
    pub method: HTTPMethod,
    pub path: String,
    pub handler: Arc<dyn HTTPHandler + 'static>,
}

pub type RouteMap = HashMap<String, String>;
pub type RouteHandlerMap = HashMap<String, Arc<dyn HTTPHandler + 'static>>;
