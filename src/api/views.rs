use super::super::lib;
use super::super::setting;
use std::fs;

// 首页
pub fn handle_index() -> (String, String) {
    (
        fs::read_to_string(&(setting::TEMPLATE_PATH.to_owned() + "/hello.html")).unwrap(),
        lib::handle::status(200, "OK"),
    )
}

// 404页面
pub fn handle_404() -> (String, String) {
    (String::new(), lib::handle::status(200, "OK"))
}
