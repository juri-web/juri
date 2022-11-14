use std::fmt;

#[derive(Clone, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
}

impl HTTPMethod {
    pub fn from(method: String) -> Result<Self, crate::Error> {
        let method = match method.to_uppercase().as_str() {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            _ => Err(crate::Error {
                code: 405,
                reason: "Method Not Allowed".to_string(),
            })?,
        };
        Ok(method)
    }
}

impl From<String> for HTTPMethod {
    fn from(method: String) -> Self {
        match method.to_uppercase().as_str() {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            _ => panic!("String conversion HTTPMethod enum failed"),
        }
    }
}

impl From<HTTPMethod> for String {
    fn from(method: HTTPMethod) -> Self {
        match method {
            HTTPMethod::GET => String::from("GET"),
            HTTPMethod::POST => String::from("POST"),
        }
    }
}

impl fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            HTTPMethod::GET => String::from("GET"),
            HTTPMethod::POST => String::from("POST"),
        };
        write!(f, "{}", method_str)
    }
}
