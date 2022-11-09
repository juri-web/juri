use crate::{
    byte::handle_bytes,
    cache::main::init_cache,
    plugin::JuriPlugin,
    routing::{conversion_router, handle_router, Router},
    Config, JuriError, Request, Response,
};
use async_std::{net::TcpListener, prelude::*, sync::Arc};
use colored::*;
use std::{collections::HashMap, net::SocketAddr};

pub struct Server {
    addr: SocketAddr,
    plugins: Vec<Box<dyn JuriPlugin>>,
    config: Config,
}

impl Server {
    pub fn bind(addr: SocketAddr) -> Self {
        Server {
            addr,
            plugins: vec![],
            config: Config::new(),
        }
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn plugin(mut self, plugin: impl JuriPlugin) -> Self {
        self.plugins.push(Box::new(plugin));
        self
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
        let plugins = Arc::new(self.plugins);

        while let Some(stream) = incoming.next().await {
            let mut stream = stream?;

            loop {
                let router = Arc::clone(&router);
                let plugins = Arc::clone(&plugins);

                match handle_bytes(&mut stream).await {
                    Ok(mut request) => {
                        let peer_addr = stream.peer_addr().unwrap().ip();
                        println!(
                            "{}: Request {} {} {}",
                            "INFO".green(),
                            request.method,
                            request.path,
                            peer_addr
                        );

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
                            Some(response) => response,
                            None => match handle_router(&mut request, router) {
                                Some(fun) => {
                                    let response: crate::Result<Response> = fun(&request);
                                    match response {
                                        Ok(response) => response,
                                        Err(err) => match err {
                                            JuriError::CustomError(_) => (response_500)(&request),
                                            JuriError::ResponseError(response) => response,
                                        },
                                    }
                                }
                                None => (response_404)(&request),
                            },
                        };

                        for plugin in plugins.iter() {
                            plugin.response(&request, &mut response);
                        }

                        println!(
                            "{}: Response {} {} {}",
                            "INFO".green(),
                            request.method,
                            request.path,
                            response.status_code
                        );
                        let response_str = response.get_response_str();
                        stream.write(response_str.as_bytes()).await.unwrap();
                        stream.flush().await.unwrap();

                        if request.version != "1.1" {
                            break;
                        }
                        if let Some(connection) = request.header("Connection") {
                            if connection != "keep-alive" {
                                break;
                            } else {
                                //TODO 临时不知持 keep-alive
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("{}: {:?}", "ERROR".red(), e);
                        break;
                    }
                };
            }
        }
        Ok(())
    }
}

fn response_404(_request: &Request) -> Response {
    Response {
        status_code: 404,
        contents: "<h1>404</h1>".to_string(),
        headers: HashMap::from([(
            "Content-Type".to_string(),
            "text/html;charset=utf-8\r\n".to_string(),
        )]),
    }
}

fn response_500(_request: &Request) -> Response {
    Response {
        status_code: 500,
        contents: "<h1>500</h1>".to_string(),
        headers: HashMap::from([(
            "Content-Type".to_string(),
            "text/html;charset=utf-8\r\n".to_string(),
        )]),
    }
}
