use super::super::lib;

// 首页
pub fn handle_index() -> (String, String) {
    (
        lib::main::template_file_return("/hello.html"),
        lib::main::status(200, "OK"),
    )
}

// 404页面
pub fn handle_404() -> (String, String) {
    (String::new(), lib::main::status(200, "OK"))
}
