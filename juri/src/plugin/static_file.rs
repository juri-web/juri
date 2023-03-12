use crate::{
    plugin::JuriPlugin, routing::match_route_params, HTTPMethod, Headers, Request, Response,
};
use chrono::prelude::*;
use std::{collections::HashMap, path::PathBuf};

pub struct StaticFilePlugin {
    config: HashMap<String, Vec<PathBuf>>,
    _is_enable_e_tag: bool,
    is_enable_last_modified: bool,
}

impl StaticFilePlugin {
    pub fn new(mut config: HashMap<&str, Vec<PathBuf>>) -> Self {
        let config = config
            .drain()
            .map(|(url, dirs)| (format!("{}{}", url, r"/(.+)"), dirs))
            .collect();

        StaticFilePlugin {
            config,
            _is_enable_e_tag: false,
            is_enable_last_modified: false,
        }
    }

    pub fn last_modified(&mut self, is_enable: bool) {
        self.is_enable_last_modified = is_enable;
    }

    fn find_file_path(&self, re_url: &String, url_path: &String) -> Option<PathBuf> {
        let dirs = self.config.get(re_url)?;
        for dir in dirs.iter() {
            let Some(dir) = dir.to_str() else {
                continue;
            };
            let file_path = PathBuf::from(format!("{dir}/{url_path}"));
            // let mut file_path = dir.clone();
            // let url_paths: Vec<_> =  url_path.split("/").collect();
            // for url_path in url_paths.iter() {
            //     file_path = file_path.join(url_path);
            // }
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
                            let metadata = file_path.metadata().expect("metadata call failed");
                            let modified = metadata
                                .modified()
                                .expect("file modified time acquisition failed");
                            let modified: DateTime<Utc> = modified.into();
                            if self.is_enable_last_modified {
                                if let Some(time) = request.header("If-Modified-Since") {
                                    let modified_since = DateTime::parse_from_str(
                                        &time,
                                        "%a, %d %b %Y %H:%M:%S GMT",
                                    )
                                    .expect("If-Modified-Since acquisition failed");
                                    if modified_since.eq(&modified) {
                                        return Some(Response {
                                            status_code: 304,
                                            ..Default::default()
                                        });
                                    }
                                }
                            }
                            let modified = modified.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
                            let mut headers = Headers::default();
                            headers.insert("Last-Modified", &modified);
                            return Some(Response {
                                body: crate::ResponseBody::Path(file_path),
                                headers,
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
