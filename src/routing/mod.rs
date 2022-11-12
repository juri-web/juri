use crate::{request::HTTPMethod, Request, Response};
use async_std::sync::Arc;
use regex::Regex;
use std::collections::HashMap;
type HandleFn = fn(request: &Request) -> crate::Result<Response>;
pub type Route = (HTTPMethod, String, HandleFn);

pub struct Router {
    get: Vec<Route>,
    post: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            get: vec![],
            post: vec![],
        }
    }

    pub fn get(&mut self, path: &str, handle: HandleFn) {
        self.get.push((HTTPMethod::GET, path.to_string(), handle));
    }

    pub fn post(&mut self, path: &str, handle: HandleFn) {
        self.post.push((HTTPMethod::GET, path.to_string(), handle));
    }
}

pub type MatchRoute = (String, Vec<String>, HandleFn);
pub struct MatchRouter {
    pub get: Vec<MatchRoute>,
    pub post: Vec<MatchRoute>,
}

pub fn conversion_router(router: Router) -> MatchRouter {
    MatchRouter {
        get: conversion_route_list(&router.get),
        post: conversion_route_list(&router.post),
    }
}

pub fn conversion_route_list(route_list: &Vec<Route>) -> Vec<MatchRoute> {
    if route_list.len() == 0 {
        return vec![];
    }
    let mut not_params_list = Vec::<MatchRoute>::new();
    let mut params_list = Vec::<MatchRoute>::new();
    for route in route_list {
        let path_split_list: Vec<&str> = route.1.split("/:").collect();
        if path_split_list.len() == 1 {
            not_params_list.push((format!(r"^{}$", path_split_list[0]), vec![], route.2));
        } else {
            let mut path_re = String::from("");
            let mut path_params: Vec<String> = vec![];
            for (index, path) in path_split_list.iter().enumerate() {
                if index == 0 {
                    path_re.push_str(path);
                } else if let Some(index) = path.find('/') {
                    path_params.push(path[..index].to_string());
                    path_re.push_str(format!("{}{}", r"/([^/]*?)", &path[index..]).as_str());
                } else {
                    path_params.push(path.to_string());
                    path_re.push_str(r"/([^/]*?)");
                }
            }
            params_list.push((format!(r"^{}$", path_re), path_params, route.2));
        }
    }
    not_params_list.sort_by(|a, b| b.0.cmp(&a.0));
    params_list.sort_by(|a, b| b.0.cmp(&a.0));
    not_params_list.append(&mut params_list);
    not_params_list
}

pub fn handle_router(request: &mut Request, router: Arc<MatchRouter>) -> Option<HandleFn> {
    let route_list;

    match request.method {
        HTTPMethod::GET => route_list = &router.get,
        HTTPMethod::POST => route_list = &router.post,
    }

    for route in route_list {
        if let Some(map) = match_router_path(&route, request.path.clone()) {
            request.params_map = map;
            return Some(route.2);
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
