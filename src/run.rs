use regex::Regex;

use crate::router::{handle_router, MatchRoute, MatchRouter, Route, Router};
use crate::thread::ThreadPool;
use crate::{Request, Response};
use std::net::TcpListener;
use std::sync::Arc;

pub struct Juri {
    router: Router,
}

impl Juri {
    pub fn new() -> Juri {
        let router = Router {
            get: [].to_vec(),
            post: [].to_vec(),
        };
        Juri { router }
    }
    pub fn run(self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(12);
        let router = Arc::new(conversion_router(self.router));
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let router = Arc::clone(&router);
            pool.execute(move || {
                let request = Request::new(&mut stream);

                if let Some(fun) = handle_router(&request, router) {
                    let response = fun(request);
                    response.write(&mut stream);
                }
            });
        }
    }

    pub fn get(&mut self, path: &str, handle: fn(request: Request) -> Response) {
        self.router.get.push((path.to_string(), handle))
    }

    pub fn post(&mut self, path: &str, handle: fn(request: Request) -> Response) {
        self.router.post.push((path.to_string(), handle))
    }
}

fn conversion_router(router: Router) -> Router {
    Router {
        get: conversion_route_list(&router.get),
        post: conversion_route_list(&router.post),
    }
}

fn conversion_route_list(route_list: &Vec<Route>) -> Vec<Route> {
    // let mut not_params_list = Vec::<MatchRoute>::new();
    // let mut params_list = Vec::<MatchRoute>::new();
    for route in route_list {
        let path_split_list: Vec<&str> = route.0.split("/:").collect();
        let mut path_re = String::from("");
        for (index, path) in path_split_list.iter().enumerate() {
            if index == 0 {
                path_re.push_str(path);
            } else {
                // path.i/
                let re = Regex::new(r"^(.*?)(?=/|$)").unwrap();
                let path = re.replace(path, "/(.*?)");
                path_re.push_str(&path.into_owned());
            }
        }
        println!("{} {:?} {}", route.0, path_split_list, path_re);

    }
    route_list.clone()
}
