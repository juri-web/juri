mod route;
mod ws_route;

pub use route::{Route, RouteHandlerMap, RouteMap};
pub use ws_route::{WSRoute, WSRouterHandlerMap};

pub enum RouteOrWSRoute {
    COMMON(Route),
    WS(WSRoute),
}
