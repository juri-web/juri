## 静态文件插件

加载静态文件插件

使用签名如下

```rust
impl StaticFilePlugin {
    /// HashMap 的 key 代表匹配 URL 的前缀，value 代表匹配 的文件路径(为多个文件路径)
    pub fn new(config: HashMap<&str, Vec<PathBuf>>>) -> Self;
}
```

例：

key 为 `/static`， value 为伪代码 `["/home", "/www/site"]`，URL 为 `/static/js/index.js` 时

会先查找 `/home/js/index.js`，然后在查找 `/www/site/js/index.js`，最后匹配失败

## 使用

```rust
use juri::plugin::StaticFilePlugin;
use std::{env, collections::HashMap};

let current_dir = env::current_dir().unwrap();

let static_file_plugin = StaticFilePlugin::new(HashMap::from([(
        "/static",
        vec![current_dir.join("web-socket").join("static")],
    )]));

juri::Server::bind(addr).plugin(static_file_plugin);
```
