use serde_derive::{Deserialize, Serialize};
use std::fs;

pub static TEMPLATE_PATH: &str = "./examples/template";
// 首页
pub fn handle_index(context: juri::Context) {
    println!("query a={}", context.query("a", "1"));
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/hello.html")).unwrap();
    context.string(200, &content);
}

#[derive(Deserialize, Serialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn handle_params(context: juri::Context) {
    let point = Point { x: 2, y: 3 };
    let content = serde_json::to_string(&point).unwrap();
    context.json(200, &content);
}
