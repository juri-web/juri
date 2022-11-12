use super::{MatchRoute, MatchRouter, Route};
use crate::Router;

pub fn conversion_router(router: Router) -> MatchRouter {
    MatchRouter {
        get: conversion_route_list(&router.get),
        post: conversion_route_list(&router.post),
    }
}

fn conversion_route_list(route_list: &Vec<Route>) -> Vec<MatchRoute> {
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
