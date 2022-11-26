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
pub use response::{Response, ResponseBody};
pub use routing::{Route, Router};
pub use server::Server;
pub use web_socket::WebSocket;

pub use async_std::main;
pub use juri_macros::get;
pub use juri_macros::post;
