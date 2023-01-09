use crate::Response;

pub type Result<T> = std::result::Result<T, ResponseAndError>;

#[derive(Debug)]
pub struct Error {
    pub code: u16,
    pub reason: String,
}

#[derive(Debug)]
pub enum ResponseAndError {
    Error(Error),
    Response(Response),
}

impl From<Response> for ResponseAndError {
    fn from(e: Response) -> Self {
        ResponseAndError::Response(e)
    }
}

impl From<Error> for ResponseAndError {
    fn from(e: Error) -> Self {
        ResponseAndError::Error(e)
    }
}
impl std::fmt::Display for ResponseAndError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ResponseAndError::Error(_) => write!(f, "ResponseAndError::Error"),
            ResponseAndError::Response(_) => write!(f, "ResponseAndError::Response"),
        }
    }
}
