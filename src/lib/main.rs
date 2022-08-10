use super::handle::handle_connection;
use super::thread::ThreadPool;
use std::net::TcpListener;
pub type Route = (String, fn() -> (String, String));
#[derive(Clone)]
pub struct Router {
    pub get: Vec<Route>,
    pub post: Vec<Route>,
}

pub struct Junior {
    router: Router,
}

impl Junior {
    pub fn new() -> Junior {
        let router = Router {
            get: [].to_vec(),
            post: [].to_vec(),
        };
        Junior { router }
    }
    pub fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(12);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let router = self.router.clone();
            pool.execute(|| {
                handle_connection(stream, router);
            });
        }
    }

    pub fn get(&mut self, path: &str, handle: fn() -> (String, String)) {
        self.router.get.push((path.to_string(), handle))
    }

    pub fn post(&mut self, path: &str, handle: fn() -> (String, String)) {
        self.router.post.push((path.to_string(), handle))
    }
}
