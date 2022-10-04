use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Request {
    pub method: String,
    pub full_path: String,
    pub version: String,
    pub path: String,
    pub header_map: HashMap<String, String>,
    pub params_map: HashMap<String, String>,
    query_str: String,
    pub hash: String,
    pub body_bytes: Vec<u8>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: "".to_string(),
            full_path: "".to_string(),
            version: "".to_string(),
            path: "".to_string(),
            header_map: HashMap::new(),
            params_map: HashMap::new(),
            query_str: "".to_string(),
            hash: "".to_string(),
            body_bytes: Vec::<u8>::new(),
        }
    }

    pub fn set_full_path(&mut self, full_path: String) {
        let (path, query_str, hash) = handle_full_path(full_path);
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

        if let Some(value) = self.header_map.get(key) {
            return Some(value.to_string());
        }

        None
    }

    pub fn form_data(self) {
        let body = String::from_utf8(self.body_bytes.to_vec()).unwrap();
        println!("{:?} {} {:?}", self.header_map, self.body_bytes.len(), body);
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
