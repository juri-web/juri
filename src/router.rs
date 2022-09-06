use regex::Regex;

use crate::{response::ResultResponse, Request, Response};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub enum HandleFn {
    Result(fn(request: Request) -> ResultResponse<Response>),
    Response(fn(request: Request) -> Response),
}

pub type Route = (String, HandleFn);
#[derive(Clone)]
pub struct Router {
    pub get: Vec<Route>,
    pub post: Vec<Route>,
}

pub type MatchRoute = (String, Vec<String>, HandleFn);
#[derive(Clone)]
pub struct MatchRouter {
    pub get: Vec<MatchRoute>,
    pub post: Vec<MatchRoute>,
}

pub fn handle_router(request: &mut Request, router: Arc<MatchRouter>) -> Option<HandleFn> {
    let route_list;
    if request.method == "GET" {
        route_list = Some(&router.get);
    } else if request.method == "POST" {
        route_list = Some(&router.post);
    } else {
        route_list = None
    }
    if let Some(route_list) = route_list {
        for route in route_list {
            if let Some(map) = match_router_path(route, request.path.clone()) {
                request.params_map = map;
                return Some(route.2.clone());
            }
        }
    }
    None
}

fn match_router_path(route: &MatchRoute, path: String) -> Option<HashMap<String, String>> {
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
