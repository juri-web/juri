## http service

# example

```rust
use juri::Juri;

fn handle_index(request: Request) -> Response {
    Response::html_str("Hello Juri")
}

fn main() {
    let mut router = Juri::new();
    router.get("/", handle_index);
    router.run("127.0.0.1:7878");
}
```