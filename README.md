<p align="center">
    <a href="https://github.com/luoxiaozero/juri" target="_blank">
        <img src="https://repository-images.githubusercontent.com/515388328/57b059d4-f581-471d-bb00-8bdd129912d2" alt="juri logo" width="180"/>
    </a>
</p>
<br/>
<p align="center">
  <a href="https://crates.io/crates/juri"><img src="https://img.shields.io/crates/v/juri" alt="crates package"></a>
</p>
<br/>

# Juri

HTTP Framework

## Getting Started

Please refer to [Juri Document](https://luoxiaozero.github.io/juri)

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
