/// https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Basics_of_HTTP/MIME_types
pub fn extension_to_mime(extension: &str) -> Option<&str> {
    match extension {
        "txt" => Some("text/plain"),
        "html" => Some("text/html"),
        "css" => Some("text/css"),
        "js" => Some("text/javascript"),

        "png" => Some("image/png"),
        "jpg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        "svg" => Some("image/svg+xml"),
        _ => None,
    }
}