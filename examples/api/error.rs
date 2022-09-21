use juri::{JuriError::*, Request, Response};

fn error(flag: bool) -> juri::Result<String> {
    if flag {
        Ok("flag: true".to_string())
    } else {
        Err(ResponseError(Response::html_str("flag: false")))
    }
}

pub fn handle_error_mode(request: Request) -> juri::Result<Response> {
    let flag = request.query("flag").map_or(false, |_v| true);
    let point = error(flag)?;

    Ok(Response::html_str(&point))
}
