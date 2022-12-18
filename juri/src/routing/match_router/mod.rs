mod route;
mod router;

use regex::Regex;
pub use router::{MatchRouteHandler, MatchRouter};
use std::collections::HashMap;

pub fn match_route_params(
    re: String,
    params: Vec<String>,
    path: String,
) -> Option<HashMap<String, String>> {
    let re = Regex::new(&re).unwrap();
    let caps = re.captures(&path);
    if let Some(caps) = caps {
        let mut map = HashMap::<String, String>::new();
        for (index, key) in params.iter().enumerate() {
            if let Some(value) = caps.get(index + 1) {
                map.insert(key.to_string(), value.as_str().to_string());
            }
        }
        return Some(map);
    }
    None
}
