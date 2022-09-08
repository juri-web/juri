use std::result;
use crate::JuriError;

pub type Result<T> = result::Result<T, JuriError>;
