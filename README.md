## http service

# example

```rust
use juri::{Request, Response, Router};
use std::net::SocketAddr;

fn handle_index(_request: &Request) -> juri::Result<Response> {
    Ok(Response::html_str("Hello Juri"))
}

#[juri::main]
async fn main() {
    let mut router = Router::new();
    router.get("/", handle_index);
    router.post("/", handle_index);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    juri::Server::bind(addr).server(router).await.unwrap();
}
```

## 路由匹配

```rust
// 匹配 /one
router.get("/one", handle_index);

// 匹配 /one/two
// 通过 `request.param("chapters")` 来获取 two
router.get("/one/:chapters", handle_index);

// 匹配 /one/two/three
router.get("/one/:chapters/three", handle_index);

// 匹配 /one/two，/one/two/three
// 通过 `request.param("chapters")` 来获取 /one/two -> two，/one/two/three -> two/three
router.get("/one/:chapters+", handle_index);
```
