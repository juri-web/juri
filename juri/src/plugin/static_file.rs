use crate::{plugin::JuriPlugin, routing::match_route_params, HTTPMethod, Request, Response};
use std::{collections::HashMap, path::PathBuf};

pub struct StaticFilePlugin {
    config: HashMap<String, Vec<PathBuf>>,
}

impl StaticFilePlugin {
    pub fn new(mut config: HashMap<&str, Vec<PathBuf>>) -> Self {
        let config = config
            .drain()
            .map(|(url, dirs)| (format!("{}{}", url, r"/(.+)"), dirs))
            .collect();

        StaticFilePlugin { config }
    }

    fn find_file_path(&self, re_url: &String, url_path: &String) -> Option<PathBuf> {
        let dirs = self.config.get(re_url)?;
        for dir in dirs.iter() {
            let Some(dir) = dir.to_str() else {
                continue;
            };
            let file_path = PathBuf::from(format!("{dir}/{url_path}"));
            if file_path.exists() && file_path.is_file() {
                return Some(file_path);
            }
        }
        None
    }
}

impl JuriPlugin for StaticFilePlugin {
    fn request(&self, request: &mut Request) -> Option<Response> {
        if request.method == HTTPMethod::GET {
            for re_url in self.config.keys() {
                if let Some(params_map) = match_route_params(
                    re_url.clone(),
                    vec!["url_path".to_string()],
                    request.path.clone(),
                ) {
                    if let Some(url_path) = params_map.get("url_path") {
                        if let Some(file_path) = self.find_file_path(re_url, url_path) {
                            return Some(Response {
                                body: crate::ResponseBody::Path(file_path),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }

        None
    }

    fn response(&self, _request: &Request, _response: &mut Response) {}
}

#[test]
fn test_file_path() {
    let path = PathBuf::from("/aa/");
    println!("{:#?}", path.to_str());

    let path = path.join("bb/cc.js");
    println!("{:#?}", path.to_str());
}
