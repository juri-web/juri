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

pub use config::Config;
pub use error::{Error, ResponseAndError, Result};
pub use plugin::{JuriPlugin, StaticFilePlugin};
pub use request::{Request, HTTPMethod};
pub use response::{Response, ResponseBody};
pub use routing::{Router, Route};
pub use server::Server;

pub use juri_macros::get;