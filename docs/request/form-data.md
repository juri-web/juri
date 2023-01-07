# FormData

```rust
/// 获取单个文件，返回 Option 类型
request.file("file");
/// 获取多个 name 值一样的文件 ，返回 Vec 类型
request.files("file");
```

```rust
let file = request.file("file").unwrap();

/// 打开缓存文件
file.open();

/// 获取缓存文件大小
file.file_size();

/// 获取文件类型
file.file_type();
```

## 复制缓存文件到指定路径

```rust
let file = request.file("file").unwrap();
let file_name = file.file_name.clone().unwrap();
let path = format!("/home/upload/{}", file_name);
let path = Path::new(&path);

file.copy(path).unwrap();
```