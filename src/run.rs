use crate::router::{handle_router, Router};
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
    pub fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(12);
        let router = Arc::new(self.router.clone());
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
