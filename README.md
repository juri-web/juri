## http service

# example

```rust
use juri::{Request, Response, Router};
use std::net::SocketAddr;

fn handle_index(_request: &Request) -> juri::Result<Response> {
    Ok(Response::html_str("Hello Juri"))
}

#[async_std::main]
async fn main() {
    let mut router = Router::new();
    router.get("/", handle_index);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr).server(router).await.unwrap();
}
```