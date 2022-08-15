use super::handle::{handle_connection, Context};
use super::router::Router;
use super::thread::ThreadPool;
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
            let stream = stream.unwrap();
            let router = Arc::clone(&router);
            pool.execute(|| {
                handle_connection(stream, router);
            });
        }
    }

    pub fn get(&mut self, path: &str, handle: fn(context: Context)) {
        self.router.get.push((path.to_string(), handle))
    }

    pub fn post(&mut self, path: &str, handle: fn(context: Context)) {
        self.router.post.push((path.to_string(), handle))
    }
}
