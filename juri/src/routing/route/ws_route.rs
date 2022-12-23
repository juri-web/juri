use crate::web_socket::WSHandler;
use std::{sync::Arc, collections::HashMap};


#[derive(Clone)]
pub struct WSRoute {
    pub path: String,
    pub handler: Arc<dyn WSHandler + 'static>,
}

pub type WSRouterHandlerMap = HashMap<String, Arc<dyn WSHandler + 'static>>;