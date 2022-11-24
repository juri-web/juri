use super::{MatchRoute, MatchRouter, Route};
use crate::{Router, HTTPMethod};

fn route_summary_get(router: &Router) -> Vec<Route> {
    let mut route_list;
    if let Some(root_path) = &router.root {
        route_list = vec![];
        for route in router.get.iter() {
            route_list.push((HTTPMethod::GET, format!("{}{}", root_path, route.1), route.2))
        }
    } else {
        route_list = router.get.clone();
    }
    
    for router in router.router.iter() {
        route_list.append(&mut route_summary_get(router));
    }
    route_list
}

fn route_summary_post(router: &Router) -> Vec<Route> {
    let mut route_list;
    if let Some(root_path) = &router.root {
        route_list = vec![];
        for route in router.post.iter() {
            route_list.push((HTTPMethod::POST, format!("{}{}", root_path, route.1), route.2))
        }
    } else {
        route_list = router.post.clone();
    }

    for router in router.router.iter() {
        route_list.append(&mut route_summary_get(router));
    }
    route_list
}

pub fn conversion_router(router: Router) -> MatchRouter {
    MatchRouter {
        get: conversion_route_list(&route_summary_get(&router)),
        post: conversion_route_list(&route_summary_post(&router)),
    }
}

fn conversion_route_list(route_list: &Vec<Route>) -> Vec<MatchRoute> {
    if route_list.is_empty() {
        return vec![];
    }

    let mut not_params_list = Vec::<MatchRoute>::new();
    let mut params_list = Vec::<MatchRoute>::new();

    for route in route_list {
        let match_route = conversion_route(route);
        if match_route.1.is_empty() {
            not_params_list.push(match_route);
        } else {
            params_list.push(match_route);
        }
    }

    not_params_list.sort_by(|a, b| b.0.cmp(&a.0));
    params_list.sort_by(|a, b| b.0.cmp(&a.0));
    not_params_list.append(&mut params_list);
    not_params_list
}

fn conversion_route(route: &Route) -> MatchRoute {
    let path_split_list: Vec<&str> = route.1.split("/:").collect();
    if path_split_list.len() == 1 {
        (format!(r"^{}$", path_split_list[0]), vec![], route.2)
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
                if path.ends_with("+") {
                    path_params.push(path[..path.len() - 1].to_string());
                    path_re.push_str(r"/(.+)");
                    break;
                } else {
                    path_params.push(path.to_string());
                    path_re.push_str(r"/([^/]*?)");
                }
            }
        }
        (format!(r"^{}$", path_re), path_params, route.2)
    }
}

#[test]
fn test_conversion_route_the_path() {
    use crate::{request::HTTPMethod, Request, Response};

    fn handle_index(_request: &Request) -> crate::Result<Response> {
        Ok(Response::html_str("Hello Juri"))
    }

    let match_route = conversion_route(&(HTTPMethod::GET, "/aa".to_string(), handle_index));
    assert_eq!(match_route.0, "^/aa$");

    let match_route = conversion_route(&(HTTPMethod::GET, "/aa/:bb".to_string(), handle_index));
    assert_eq!(match_route.0, "^/aa/([^/]*?)$");

    let match_route = conversion_route(&(HTTPMethod::GET, "/aa/:bb/cc".to_string(), handle_index));
    assert_eq!(match_route.0, "^/aa/([^/]*?)/cc$");

    let match_route = conversion_route(&(HTTPMethod::GET, "/aa/:bb+".to_string(), handle_index));
    assert_eq!(match_route.0, "^/aa/(.+)$");
}
