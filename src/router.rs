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
        let path_split_list: Vec<&str> = route.0.split("/:").collect();
        if path_split_list.len() == 1 {
            not_params_list.push((
                format!(r"^{}$", path_split_list[0]),
                vec![],
                route.1.clone(),
            ));
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
            params_list.push((format!(r"^{}$", path_re), path_params, route.1.clone()));
        }
    }
    not_params_list.sort_by(|a, b| b.0.cmp(&a.0));
    params_list.sort_by(|a, b| b.0.cmp(&a.0));
    not_params_list.append(&mut params_list);
    not_params_list
}
