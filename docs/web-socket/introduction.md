## WebSocket 是啥

WebSocket 是一种网络传输协议，可在单个 TCP 连接上进行全双工通信，位于 OSI 模型的应用层。具体可以参考 [维基百科](https://zh.m.wikipedia.org/zh-hans/WebSocket)

JavaScript 的 WebSocket 使用可以参考 [MDN 文档](https://developer.mozilla.org/zh-CN/docs/Web/API/WebSocket)

## 使用

```rust
use juri::{get, Request, Response, web_socket::{Message, RequestExt, WSResponse}};

#[get("/ws", ws)]
pub fn handle_ws(&request: Request) -> juri::Result<WSResponse> {
    /// 升级为 ws, 成功时返回 WSResponse
    let mut ws = request.upgrader().unwrap();

    // 传入 ws 处理逻辑
    ws.on(|mut stream| async move {
        loop {
            let message = stream.read().await.unwrap();
            match message {
                Message::Text(text) => {
                    stream.send(WSMessage::Text("hi".to_string())).await.unwrap();
                },
                Message::Binary(_) => todo!(),
                Message::Ping(_) => todo!(),
                Message::Pong(_) => todo!(),
                Message::Close => {
                    return;
                }
            }
        }
    });

    /// 返回 WSResponse
    Ok(ws)

    /// 也可以自定义返回
    // Ok(WSResponse::new(Response::html("")))
}
```
