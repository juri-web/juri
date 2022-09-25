use crate::{router::HandleFn, JuriError, Request, Response};

pub fn handle_fn(request: Request, fun: HandleFn) -> crate::Result<Response> {
    let response = match fun {
        HandleFn::Result(fun) => {
            let response = fun(request);
            match response {
                Ok(response) => response,
                Err(response) => response,
            }
        }
        HandleFn::Response(fun) => fun(request),
        HandleFn::Error(fun) => {
            let response = fun(request);
            match response {
                Ok(response) => response,
                Err(err) => match err {
                    JuriError::CustomError(e) => Err(e)?,
                    JuriError::ResponseError(response) => response,
                },
            }
        }
    };
    Ok(response)
}

pub trait JuriPlugin: Send + Sync + 'static {
    fn request(&self, request: &mut Request) -> Option<Response>;
    fn response(&self, response: &mut Response);
}
