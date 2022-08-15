use std::fs;

pub static TEMPLATE_PATH: &str = "./examples/template";
// 首页
pub fn handle_index(context: juri::Context) {
    println!("query a={}", context.query("a", "1"));
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/hello.html")).unwrap();
    context.string(200, &content);
}

pub fn handle_params(context: juri::Context) {
    let content = fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/404.html")).unwrap();
    context.string(200, &content);
}
