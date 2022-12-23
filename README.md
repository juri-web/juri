<p align="center">
    <a href="https://github.com/luoxiaozero/juri" target="_blank">
        <span
            style="
                position: relative;
                width: 110px;
                height: 110px;
                border-radius: 38px;
                border: 9px solid #333;
                display: inline-flex;
                justify-content: center;
                align-items: center;
            "
        >
            <span
                style="
                    position: absolute;
                    top: -16px;
                    right: -16px;
                    width: 55px;
                    height: 55px;
                    background-color: #ec2b24;
                    border-radius: 50%;
                "
            ></span>
        </span>
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
