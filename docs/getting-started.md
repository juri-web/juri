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

## 总览

Juri 是一个 Web 服务器框架。

[Github 地址](https://github.com/luoxiaozero/juri)

## 安装

[Crates 地址](https://crates.io/crates/juri)

cargo:


```shell
cargo add juri 
cargo add async-std
```

或者

在 Cargo.toml 文件里添加 `juri = "0.4.0-alpha.1"`

## 使用

最小 Demo:

```rust,noplayground
use juri::{Request, Response, Router, handler};
use std::net::SocketAddr;

#[handler]
fn handle_index(_request: &Request) -> juri::Result<Response> {
    Ok(Response::html("Hello Juri"))
}

#[juri::main]
async fn main() {
    let mut router = Router::new();

    router.at("/").get(api::views::handle_index);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr).server(router).await.unwrap();
}
```