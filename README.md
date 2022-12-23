# HTTP Framework

## Getting Started

Please refer to [juri document](https://luoxiaozero.github.io/juri)

## Example

```rust
use juri::{Request, Response, Router, handler};
use std::net::SocketAddr;

#[handler]
fn handle_index(_request: &Request) -> juri::Result<Response> {
    Ok(Response::html_str("Hello Juri"))
}

#[juri::main]
async fn main() {
    let mut router = Router::new();
    router.at("/").get(handle_index);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr).server(router).await.unwrap();
}
```

## License

[MIT](./LICENSE) License