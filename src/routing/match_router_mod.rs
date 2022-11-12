use super::HandleFn;
use crate::{request::HTTPMethod, Request};
use regex::Regex;
use std::{collections::HashMap, sync::Arc};

pub type MatchRoute = (String, Vec<String>, HandleFn);
pub struct MatchRouter {
    pub get: Vec<MatchRoute>,
    pub post: Vec<MatchRoute>,
}

pub fn match_route(request: &mut Request, router: Arc<MatchRouter>) -> Option<HandleFn> {
    let route_list;

    match request.method {
        HTTPMethod::GET => route_list = &router.get,
        HTTPMethod::POST => route_list = &router.post,
    }

    for route in route_list {
        if let Some(map) = match_route_path(&route, request.path.clone()) {
            request.params_map = map;
            return Some(route.2);
        }
    }
    None
}

fn match_route_path(route: &MatchRoute, path: String) -> Option<HashMap<String, String>> {
    let re = Regex::new(route.0.as_str()).unwrap();
    let caps = re.captures(&path);
    if let Some(caps) = caps {
        let mut map = HashMap::<String, String>::new();
        for (index, key) in route.1.iter().enumerate() {
            if let Some(value) = caps.get(index + 1) {
                map.insert(key.to_string(), value.as_str().to_string());
            }
        }
        return Some(map);
    }
    None
}
