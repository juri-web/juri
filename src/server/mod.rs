use crate::{
    byte::handle_bytes,
    cache::main::init_cache,
    routing::{conversion_router, handle_router, Router},
    JuriError, Request, Response,
};
use async_std::{net::TcpListener, prelude::*, sync::Arc};
use colored::*;
use std::{collections::HashMap, net::SocketAddr};

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn bind(addr: SocketAddr) -> Self {
        Server { addr }
    }

    pub async fn server(
        self,
        router: Router,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        init_cache();
        let listener = TcpListener::bind(self.addr).await?;
        println!(
            "{}: listener port http://{} start",
            "Juri".green(),
            self.addr
        );
        let mut incoming = listener.incoming();
        let router = Arc::new(conversion_router(router));

        while let Some(stream) = incoming.next().await {
            let mut stream = stream?;
            let router = Arc::clone(&router);

            match handle_bytes(&mut stream).await {
                Ok(mut request) => {
                    let method = request.method.clone();
                    let path = request.path.clone();
                    let peer_addr = stream.peer_addr().unwrap().ip();
                    println!(
                        "{}: Request {} {} {}",
                        "INFO".green(),
                        request.method,
                        request.path,
                        peer_addr
                    );

                    let response = match handle_router(&mut request, router) {
                        Some(fun) => {
                            let request_copy = request.clone();
                            let response: crate::Result<Response> = fun(request);
                            match response {
                                Ok(response) => response,
                                Err(err) => match err {
                                    JuriError::CustomError(_) => (response_500)(request_copy),
                                    JuriError::ResponseError(response) => response,
                                },
                            }
                        }
                        None => (response_404)(request),
                    };

                    println!(
                        "{}: Response {} {} {}",
                        "INFO".green(),
                        method,
                        path,
                        response.status_code
                    );
                    let response_str = response.get_response_str();
                    stream.write(response_str.as_bytes()).await.unwrap();
                    stream.flush().await.unwrap();
                }
                Err(e) => println!("{}: {:?}", "ERROR".red(), e),
            };
        }
        Ok(())
    }
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
