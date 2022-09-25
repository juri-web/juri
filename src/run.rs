use crate::byte::handle_bytes;
use crate::error::JuriError;
use crate::plugin::handle_fn;
use crate::router::{conversion_router, handle_router, HandleFn, Router};
use crate::thread::ThreadPool;
use crate::{JuriPlugin, Request, Response, ResultResponse};
use colored::*;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpListener;
use std::sync::Arc;

pub struct Juri {
    plugins: Vec<Box<dyn JuriPlugin>>,
    router: Router,
    thread_size: usize,
    response_404: fn(request: Request) -> Response,
    response_500: fn(request: Request) -> Response,
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

fn response_500(_request: Request) -> Response {
    Response {
        status_code: 500,
        contents: "<h1>500</h1>".to_string(),
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
            response_500,
            plugins: vec![],
        }
    }
    pub fn run(self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        println!("{}: listener port http://{} start", "Juri".green(), addr);
        let pool = ThreadPool::new(self.thread_size);
        println!("{}: thread size is {}", "Juri".green(), self.thread_size);
        let router = Arc::new(conversion_router(self.router));
        let plugins = Arc::new(self.plugins);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let router = Arc::clone(&router);
            let plugins = Arc::clone(&plugins);
            pool.execute(move || match handle_bytes(&mut stream) {
                Ok((headers_bytes, body_bytes)) => {
                    let mut request = Request::new(headers_bytes, body_bytes);
                    let method = request.method.clone();
                    let path = request.path.clone();
                    println!("{}: Request {} {}", "INFO".green(), method, path);

                    let mut plugin = plugins.iter();
                    let plugin_response = loop {
                        match plugin.next() {
                            Some(plugin) => {
                                let response = plugin.request(&mut request);
                                if let Some(response) = response {
                                    break Some(response);
                                }
                            }
                            None => break None,
                        }
                    };
                    let mut response = match plugin_response {
                        Some(response) => {
                           response
                        }
                        None => {
                            match handle_router(&mut request, router) {
                                Some(fun) => {
                                    // FIXME 效率问题 The efficiency problem
                                    let request_copy = request.clone();

                                    let response = handle_fn(request, fun);
                                    match response {
                                        Ok(response) => response,
                                        Err(err) => match err {
                                            JuriError::CustomError(_) => {
                                                (self.response_500)(request_copy)
                                            }
                                            JuriError::ResponseError(response) => response,
                                        },
                                    }
                                }
                                None => (self.response_404)(request),
                            }
                        },
                    };
            
                    for plugin in plugins.iter() {
                        plugin.response(&mut response);
                    }
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

    pub fn get_error_mode(
        &mut self,
        path: &str,
        handle: fn(request: Request) -> crate::Result<Response>,
    ) {
        self.router
            .get
            .push((path.to_string(), HandleFn::Error(handle)));
    }

    pub fn post_error_mode(
        &mut self,
        path: &str,
        handle: fn(request: Request) -> crate::Result<Response>,
    ) {
        self.router
            .post
            .push((path.to_string(), HandleFn::Error(handle)));
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn JuriPlugin>) {
        self.plugins.push(plugin)
    }
}
