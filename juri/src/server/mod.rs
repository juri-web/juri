use crate::{
    cache::main::init_cache,
    plugin::JuriPlugin,
    routing::{MatchRouter, Router},
    server::handle::handle_request,
    Config,
};
use async_std::{net::TcpListener, prelude::*, sync::Arc, task::spawn};
use colored::*;
use std::net::SocketAddr;
mod handle;

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
        let router = Arc::new(MatchRouter::new(router));
        let plugins = Arc::new(self.plugins);
        let config = Arc::new(self.config);

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            let router = Arc::clone(&router);
            let plugins = Arc::clone(&plugins);
            let config = Arc::clone(&config);

            spawn(handle_request(stream, router, plugins, config));
        }
        Ok(())
    }
}
