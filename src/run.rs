use crate::byte::handle_bytes;
use crate::router::{conversion_router, handle_router, HandleFn, Router};
use crate::thread::ThreadPool;
use crate::{Request, Response, ResultResponse};
use colored::*;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpListener;
use std::sync::Arc;

pub struct Juri {
    router: Router,
    thread_size: usize,
    response_404: fn(request: Request) -> Response,
}

fn response_404(_request: Request) -> Response {
    Response {
        status_code: 404,
        contents: "<h1>404</h1>".to_string(),
        headers: HashMap::from([(
            "Content-Type".to_string(),
            "text/html;charset=utf-8\r\n".to_string(),
        )]),
    }
}

impl Juri {
    pub fn new() -> Juri {
        let router = Router {
            get: [].to_vec(),
            post: [].to_vec(),
        };
        Juri {
            router,
            thread_size: 6,
            response_404,
        }
    }
    pub fn run(self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        println!("{}: listener port http://{} start", "Juri".green(), addr);
        let pool = ThreadPool::new(self.thread_size);
        println!("{}: thread size is {}", "Juri".green(), self.thread_size);
        let router = Arc::new(conversion_router(self.router));
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let router = Arc::clone(&router);
            pool.execute(move || match handle_bytes(&mut stream) {
                Ok((headers_bytes, body_bytes)) => {
                    let mut request = Request::new(headers_bytes, body_bytes);
                    let method = request.method.clone();
                    let path = request.path.clone();
                    println!("{}: Request {} {}", "INFO".green(), method, path);
                    let response = match handle_router(&mut request, router) {
                        Some(fun) => match fun {
                            HandleFn::Result(fun) => {
                                let response = fun(request);
                                let response = match response {
                                    Ok(response) => response,
                                    Err(response) => response,
                                };
                                response
                            }
                            HandleFn::Response(fun) => fun(request),
                        },
                        None => (self.response_404)(request),
                    };
                    println!(
                        "{}: Response {} {} {}",
                        "INFO".green(),
                        method,
                        path,
                        response.status_code
                    );
                    let response_str = response.get_response_str();
                    stream.write(response_str.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(e) => println!("{}: {:?}", "Juri".green(), e),
            });
        }
    }

    pub fn get(&mut self, path: &str, handle: fn(request: Request) -> Response) {
        self.router
            .get
            .push((path.to_string(), HandleFn::Response(handle)));
    }

    pub fn post(&mut self, path: &str, handle: fn(request: Request) -> Response) {
        self.router
            .post
            .push((path.to_string(), HandleFn::Response(handle)));
    }

    pub fn get_result_mode(
        &mut self,
        path: &str,
        handle: fn(request: Request) -> ResultResponse<Response>,
    ) {
        self.router
            .get
            .push((path.to_string(), HandleFn::Result(handle)));
    }

    pub fn post_result_mode(
        &mut self,
        path: &str,
        handle: fn(request: Request) -> ResultResponse<Response>,
    ) {
        self.router
            .post
            .push((path.to_string(), HandleFn::Result(handle)));
    }
}
