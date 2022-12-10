use crate::{
    byte::{handle_bytes, send_stream},
    error::ResponseAndError,
    plugin::JuriPlugin,
    routing::{match_route, MatchRouter},
    Config, Response, ResponseBody, IntoResponse,
};
use async_std::{net::TcpStream, sync::Arc};
use colored::*;
use std::collections::HashMap;

pub async fn handle_request(
    mut stream: TcpStream,
    router: Arc<MatchRouter>,
    plugins: Arc<Vec<Box<dyn JuriPlugin>>>,
    config: Arc<Config>,
) {
    loop {
        let router = Arc::clone(&router);
        let plugins = Arc::clone(&plugins);
        let config = Arc::clone(&config);

        match handle_bytes(&mut stream, &config).await {
            Ok(mut request) => {
                let peer_addr = stream.peer_addr().unwrap().ip();
                println!(
                    "{}: Request {} {} {}",
                    "INFO".green(),
                    request.method,
                    request.path,
                    peer_addr
                );

                let mut run_plugin_number: usize = 0;
                let mut plugin = plugins.iter();
                let plugin_response = loop {
                    match plugin.next() {
                        Some(plugin) => {
                            run_plugin_number += 1;
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
                    None => match match_route(&mut request, router) {
                        Some(handler) => {
                            let response = handler.call(&request).await;
                            match response {
                                Ok(response) => response.into_response(),
                                Err(err) => match err {
                                    ResponseAndError::Error(e) => Response {
                                        status_code: e.code,
                                        headers: HashMap::new(),
                                        body: ResponseBody::None,
                                    },
                                    ResponseAndError::Response(response) => response.into_response(),
                                },
                            }
                        }
                        None => Response {
                            status_code: 404,
                            headers: HashMap::new(),
                            body: ResponseBody::Text("".to_string()),
                        },
                    },
                };

                for plugin in plugins.iter().rev() {
                    if run_plugin_number == 0 {
                        break;
                    }
                    run_plugin_number -= 1;
                    plugin.response(&request, &mut response);
                }

                println!(
                    "{}: Response {} {} {}",
                    "INFO".green(),
                    request.method,
                    request.path,
                    response.status_code
                );

                send_stream(&mut stream, &config, Some(&request), &response).await;

                if !request.is_keep_alive() {
                    break;
                }
            }
            Err(e) => {
                match e.code {
                    100..=599 => {
                        let response = Response {
                            status_code: e.code,
                            headers: HashMap::new(),
                            body: ResponseBody::None,
                        };
                        send_stream(&mut stream, &config, None, &response).await;
                    }
                    _ => {
                        println!("{}: {:?}", "ERROR".red(), e);
                    }
                }
                break;
            }
        };
    }
}
