use std::fs;
pub static TEMPLATE_PATH: &str = "./examples/template";
// 首页
pub fn handle_index() -> (String, String) {
    (
        fs::read_to_string(&(TEMPLATE_PATH.to_owned() + "/hello.html")).unwrap(),
        juri::handle::status(200, "OK"),
    )
}

// 404页面
pub fn handle_404() -> (String, String) {
    (String::new(), juri::handle::status(200, "OK"))
}
