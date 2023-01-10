use crate::byte::FormData;
use regex::Regex;
use std::collections::HashMap;
mod http_method;
pub use http_method::HTTPMethod;

#[derive(Clone)]
pub struct Request {
    pub method: HTTPMethod,
    pub full_path: String,
    pub protocol_and_version: String,
    pub path: String,
    pub(crate) header_map: HashMap<String, String>,
    pub(crate) params_map: HashMap<String, String>,
    query_str: String,
    pub hash: String,
    pub body_bytes: Vec<u8>,
    pub(crate) multipart_form_data: Vec<FormData>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: HTTPMethod::GET,
            full_path: Default::default(),
            protocol_and_version: Default::default(),
            path: Default::default(),
            header_map: Default::default(),
            params_map: Default::default(),
            query_str: Default::default(),
            hash: Default::default(),
            body_bytes: Default::default(),
            multipart_form_data: Default::default(),
        }
    }
}

impl Request {
    pub fn set_full_path(&mut self, full_path: String) {
        self.full_path = full_path;
        let (path, query_str, hash) = handle_full_path(self.full_path.clone());
        self.path = path;
        self.query_str = query_str;
        self.hash = hash;
    }

    pub fn query(&self, key: &str) -> Option<String> {
        if self.query_str.is_empty() {
            return None;
        }

        let re = Regex::new(&format!(r"[\?|\&]{}=(.*?)(\&|$)", key)).unwrap();
        let caps = re.captures(&self.query_str);
        if let Some(caps) = caps {
            if let Some(value) = caps.get(1) {
                return Some(value.as_str().to_string());
            }
        }
        None
    }

    pub fn param(&self, key: &str) -> Option<String> {
        if self.params_map.is_empty() {
            return None;
        }

        if let Some(value) = self.params_map.get(key) {
            return Some(value.to_string());
        }

        None
    }

    pub fn header(&self, key: &str) -> Option<String> {
        if self.header_map.is_empty() {
            return None;
        }

        if let Some(value) = self.header_map.get(&key.to_lowercase()) {
            return Some(value.to_string());
        }

        None
    }

    pub fn cookie(&self, key: &str) -> Option<String> {
        if let Some(cookie) = self.header("Cookie") {
            let re = Regex::new(&format!(r"(\;|^)\s*{}=(.*?)\s*(\;|$)", key)).unwrap();
            let caps = re.captures(&cookie);
            if let Some(caps) = caps {
                if let Some(value) = caps.get(2) {
                    return Some(value.as_str().to_string());
                }
            }
        }
    }

    pub fn file(&self, name: &str) -> Option<FormData> {
        for form_data in self.multipart_form_data.iter() {
            if form_data.name == name {
                return Some(form_data.clone());
            }
        }
        None
    }

    pub fn files(&self, name: &str) -> Vec<FormData> {
        let mut form_data_list = vec![];
        for form_data in self.multipart_form_data.iter() {
            if form_data.name == name {
                form_data_list.push(form_data.clone());
            }
        }
        form_data_list
    }

    pub fn is_keep_alive(&self) -> bool {
        if self.protocol_and_version == "HTTP/1.1" {
            if let Some(connection) = self.header("Connection") {
                if connection == "keep-alive" {
                    return true;
                }
            }
        }

        false
    }
}

fn handle_full_path(full_path: String) -> (String, String, String) {
    let re = Regex::new(r"^(.*?)(\?.*?)?(#.*?)?$").unwrap();
    let caps = re.captures(&full_path).unwrap();
    let path = caps
        .get(1)
        .map_or("".to_string(), |m| m.as_str().to_string());
    let query_str = caps
        .get(2)
        .map_or("".to_string(), |m| m.as_str().to_string());
    let hash = caps
        .get(3)
        .map_or("".to_string(), |m| m.as_str().to_string());
    (path, query_str, hash)
}

#[cfg(test)]
mod test {
    use crate::Request;

    #[test]
    fn header() {
        let mut request = Request::default();
        request
            .header_map
            .insert("Context-Type".to_string().to_lowercase(), "hi".to_string());
        assert_eq!(request.header("context-type"), Some("hi".to_string()));
        assert_eq!(request.header("Context-type"), Some("hi".to_string()));
        assert_eq!(request.header("Context-Type"), Some("hi".to_string()));
    }
}
