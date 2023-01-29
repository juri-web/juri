use juri::{
    get,
    web_socket::{Message, WSRequestExt, WSResponse},
    Response,
};
use std::fs;

pub static TEMPLATE_PATH: &str = "./web-socket/template";

#[get("/ws", ws)]
pub fn handle_ws(request: &juri::Request) -> juri::Result<WSResponse> {
    let mut ws = request.upgrader().unwrap();

    ws.on(|mut stream| async move {
        loop {
            let message = stream.read().await.unwrap();
            match message {
                Message::Text(text) => {
                    println!("ws test: {text}");
                    stream.send(Message::Text("hi".to_string())).await.unwrap();
                }
                Message::Binary(_) => todo!(),
                Message::Ping(_) => todo!(),
                Message::Pong(_) => todo!(),
                Message::Close => {
                    return;
                }
            }
        }
    });
    Ok(ws)
}

#[get("/")]
pub fn handle_index(_request: &juri::Request) -> juri::Result<Response> {
    let content = fs::read_to_string(TEMPLATE_PATH.to_owned() + "/index.html").unwrap();
    Ok(Response::html(&content))
}
