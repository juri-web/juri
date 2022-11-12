use crate::Response;

pub type Result<T> = std::result::Result<T, JuriError>;

#[derive(Debug)]
pub struct JuriCustomError {
    pub code: u16,
    pub reason: String,
}

/// # Examples
///```
/// use juri::{Request, Response, JuriError, JuriCustomError};
/// use std::collections::HashMap;
///
/// fn get_super_error(flag: bool) -> juri::Result<()> {
///     if flag {
///         Err(JuriError::ResponseError(Response {
///             status_code: 200,
///             contents: "".to_string(),
///             headers: HashMap::new(),
///         }))
///     } else {
///         Err(JuriError::CustomError(JuriCustomError {
///             code: 1,
///             reason: "".to_string(),
///         }))
///     }
/// }
///
/// fn main() -> juri::Result<()> {
///     let temp = get_super_error(true)?;
///     let temp = get_super_error(false)?;
///     Ok(temp)
/// }
///```
#[derive(Debug)]
pub enum JuriError {
    CustomError(JuriCustomError),
    ResponseError(Response),
}

impl From<Response> for JuriError {
    fn from(e: Response) -> Self {
        JuriError::ResponseError(e)
    }
}

impl From<JuriCustomError> for JuriError {
    fn from(e: JuriCustomError) -> Self {
        JuriError::CustomError(e)
    }
}
impl std::fmt::Display for JuriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            JuriError::CustomError(_) => write!(f, "JuriError::CustomError"),
            JuriError::ResponseError(_) => write!(f, "JuriError::Response"),
        }
    }
}
