mod byte;
mod cache;
mod config;
mod error;
pub mod json;
mod plugin;
mod request;
mod response;
mod routing;
mod server;
mod web_socket;

pub use config::Config;
pub use error::{Error, ResponseAndError, Result};
pub use plugin::{JuriPlugin, StaticFilePlugin};
pub use request::{HTTPMethod, Request};
pub use response::{HTTPHandler, Response, ResponseBody};
pub use routing::{Route, RouteOrWSRoute, Router, WSRoute};
pub use server::Server;
pub use web_socket::{WSHandler, WSRequest, WSResponse};

pub use async_std::main;
pub use async_trait::async_trait;
pub use juri_macros::get;
pub use juri_macros::handler;
pub use juri_macros::post;
