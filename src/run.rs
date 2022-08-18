use crate::router::{handle_router, MatchRoute, MatchRouter, Route, Router};
use crate::thread::ThreadPool;
use crate::{Request, Response};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
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
            pool.execute(move || match handle_bytes(&mut stream) {
                Ok((headers_bytes, body_bytes)) => {
                    let mut request = Request::new(headers_bytes, body_bytes);

                    if let Some(fun) = handle_router(&mut request, router) {
                        let response = fun(request);
                        let response_str = response.get_response_str();
                        stream.write(response_str.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                }
                Err(e) => println!("{:?}", e),
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

fn handle_bytes(stream: &mut TcpStream) -> io::Result<(Vec<Vec<u8>>, Vec<u8>)> {
    // https://www.cnblogs.com/nxlhero/p/11670942.html
    // https://rustcc.cn/article?id=2b7eb30b-61ae-4a3d-96fd-fc897ab7b1e0
    let mut headers_bytes = Vec::<Vec<u8>>::new();
    let mut body_bytes = Vec::<u8>::new();
    let mut temp_header_bytes = Vec::<u8>::new();
    let mut flag_body = false;
    loop {
        let mut buffer = vec![0u8; 1024 * 4];
        let bytes_count = stream.read(&mut buffer)?;
        if bytes_count == 0 {
            break;
        } else if flag_body {
            body_bytes.append(&mut buffer);
        } else {
            let mut flag_n = false;
            let mut flag_r = false;
            let mut flag_index = 0;
            for (index, value) in buffer.iter().enumerate() {
                if flag_n {
                    if *value == 10 {
                        flag_r = true;
                    } else {
                        flag_n = false;
                    }
                }
                if *value == 13 {
                    flag_n = true;
                }
                if flag_n && flag_r {
                    if index == flag_index + 1 {
                        body_bytes.append(&mut buffer[(index + 1)..].to_vec());
                        flag_body = true;
                        break;
                    } else if temp_header_bytes.len() == 0 {
                        headers_bytes.push(buffer[flag_index..(index - 1)].to_vec().clone())
                    } else {
                        temp_header_bytes
                            .append(&mut buffer[flag_index..(index - 1)].to_vec().clone());
                        headers_bytes.push(temp_header_bytes.clone());
                        temp_header_bytes.clear()
                    }
                    flag_index = index + 1;
                    flag_n = false;
                    flag_r = false;
                }
            }
        }
        if bytes_count < 1024 * 4 {
            break;
        }
    }
    Ok((headers_bytes, body_bytes))
}

fn conversion_router(router: Router) -> MatchRouter {
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
        let path_split_list: Vec<&str> = route.0.split("/:").collect();
        if path_split_list.len() == 1 {
            not_params_list.push((format!("^{}$", path_split_list[0]), vec![], route.1));
        } else {
            let mut path_re = String::from("");
            let mut path_params: Vec<String> = vec![];
            for (index, path) in path_split_list.iter().enumerate() {
                if index == 0 {
                    path_re.push_str(path);
                } else if let Some(index) = path.find('/') {
                    path_params.push(path[..index].to_string());
                    path_re.push_str(format!("{}{}", "/(.*?)", &path[index..]).as_str());
                } else {
                    path_params.push(path.to_string());
                    path_re.push_str("/(.*?)");
                }
            }
            params_list.push((format!("^{}$", path_re), path_params, route.1));
        }
    }
    not_params_list.sort_by(|a, b| b.0.cmp(&a.0));
    params_list.sort_by(|a, b| b.0.cmp(&a.0));
    not_params_list.append(&mut params_list);
    not_params_list
}
