use juri::{Request, Response};

fn result(flag: bool) -> juri::Result<String> {
    if flag {
        Ok("mode true".to_string())
    } else {
        Err(Response::html_str("Mode false"))?
    }
}

pub fn handle_result_mode(request: &Request) -> juri::Result<Response> {
    let flag = request.query("flag").map_or(false, |_v| true);
    let point = result(flag)?;

    Ok(Response::html_str(&point))
}
