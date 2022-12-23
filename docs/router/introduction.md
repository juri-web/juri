# 入门

## 新建路由器

```rust,noplayground
use juri::Router;

let router = Router::new();
```

## 添加处理函数

目前只支持 `get` 和 `post` 两种请求方式

### At 方式

```rust,noplayground
use juri::Router;

let mut router = Router::new();

router.at("/").get(handle_fn);

/// 路径一样时，可以链式调用
router.at("/").get(handle_fn).post(handle_fn);
```

函数定义 

```rust,noplayground
use juri::{handler, Request, Response};

/// 支持同步和异步。异步时在 `fn` 前添加 `async`
#[handler]
fn handle_fn(request: &Request) -> juri::Result<Response> {
    /// body
}
```

### Route 方式

```rust,noplayground
use juri::Router;

let mut router = Router::new();

/// 由函数提供，请求方法和路径
router.route(handle_fn);
```

函数定义 

```rust,noplayground
use juri::{get, post, Request, Response};

/// 支持同步和异步。异步时在 `fn` 前添加 `async`
#[get("/")]
fn handle_fn(request: &Request) -> juri::Result<Response> {
    /// body
}

/// 支持同步和异步。异步时在 `fn` 前添加 `async`
#[post("/")]
fn handle_fn_post(request: &Request) -> juri::Result<Response> {
    /// body
}
```

## 路由器分组

```rust
use juri::Router;

let mut router = Router::new();
router.at("/").get(handle_fn);

let mut child_router = Router::new();
child_router.at("/one").get(child_handle_fn);

/// 此时，`/` 和 `/one` 都可以处理
router.router(child_router);
```

子路由添加根路径

```rust
use juri::Router;

let mut router = Router::new();
router.at("/").get(handle_fn);

let mut child_router = Router::new();
child_router.root("/hi");
child_router.at("/one").get(child_handle_fn);

/// 此时处理 `/` 和 `/hi/one`
/// 也就是路径是 `/hi/one`，才执行 `child_handle_fn` 函数
router.router(child_router);
```