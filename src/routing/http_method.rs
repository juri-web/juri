pub enum HTTPMethod {
    GET,
    POST,
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

