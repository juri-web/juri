# 开始

## 总览

Juri 是一个 Web 服务器框架。

## 安装

[Juri 项目 Crates 地址](https://crates.io/crates/juri)

cargo:


```shell
cargo add juri 
```

或者

在 Cargo.toml 文件里添加 `juri = "0.4.0"`

## 使用

最小 Demo:

```rust,noplayground
use juri::{Request, Response, Router, handler};
use std::net::SocketAddr;

#[handler]
fn handle_index(_request: &Request) -> juri::Result<Response> {
    Ok(Response::html_str("Hello Juri"))
}

#[juri::main]
async fn main() {
    let mut router = Router::new();

    router.at("/").get(api::views::handle_index);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr).server(router).await.unwrap();
}
```