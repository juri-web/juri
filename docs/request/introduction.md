## 请求（Request）

## Header

```rust
/// 获取 header
pub fn header(&self, key: &str) -> Option<String>;

/// 获取多个 key 相同的值
pub fn header_multi_value(&self, key: &str) -> Option<HeaderValues>;
```

```rust
/// 获取 cookie
pub fn cookie(&self, key: &str) -> Option<String>;
```

```rust
/// 获取路径参数
pub fn param(&self, key: &str) -> Option<String>;

/// 获取查询参数
pub fn query(&self, key: &str) -> Option<String>;
```

## FormData

[FormData](./form-data.md)

