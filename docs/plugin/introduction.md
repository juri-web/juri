## 插件

创建插件需要实现以下 JuriPlugin trait

```rust
pub trait JuriPlugin: Send + Sync + 'static {
    /// 请求拦截器
    fn request(&self, request: &mut Request) -> Option<Response>;
    /// 响应拦截器
    fn response(&self, request: &Request, response: &mut Response);
}
```

例：

```rust
use juri::plugin::JuriPlugin;

struct MyPlugin {

}

impl JuriPlugin for MyPlugin {
    fn request(&self, request: &mut Request) -> Option<Response> {
        /// 可修改请求内容

        /// 返回是 `None` 时，继续执行
        /// or
        /// 返回是 `Response` 时，拦截接下来的操作


        /// 例：插件有 `[1, 2, 3]`
        /// 插件 2 返回 `Response` 时，执行顺序为 request：1 - 2，response： 2 - 1
        /// 都为 `None` 时，执行顺序为 request：1 - 2 - 3，匹配路由，response： 3 - 2 - 1
    }

    fn response(&self, request: &Request, response: &mut Response) {
        /// 可修改请求内容和响应内容
    }
}
```

## 使用

```rust
let my_plugin = MyPlugin {};

/// 生成 `MyPlugin` 实例传入 `plugin` 函数
juri::Server::bind(addr).plugin(my_plugin);
```

## 内置插件

[静态文件插件](./plugin-static-file.md)
