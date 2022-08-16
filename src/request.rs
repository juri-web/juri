use regex::Regex;
use std::{collections::HashMap, io::Read, net::TcpStream};
/**1KB */
type ReadBuffer = [u8; 1024];
pub struct Request {
    pub method: String,
    pub full_path: String,
    pub path: String,
    pub params_map: HashMap<String, String>,
    query_str: String,
    pub hash: String,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Self {
        let mut buffer: ReadBuffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let header = String::from_utf8((&buffer).to_vec()).unwrap();
        let re = Regex::new(r"^(.*?) (.*?) (.*?)\r\n").unwrap();
        let caps = re.captures(&header).unwrap();
        let method = caps
            .get(1)
            .map_or("".to_string(), |m| m.as_str().to_string());
        let full_path = caps
            .get(2)
            .map_or("".to_string(), |m| m.as_str().to_string());

        let (path, query_str, hash) = handle_full_path(&full_path);

        Request {
            method,
            full_path,
            path,
            params_map: HashMap::new(),
            query_str,
            hash,
        }
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
}

fn handle_full_path(full_path: &String) -> (String, String, String) {
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
