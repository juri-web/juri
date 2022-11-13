use std::{path::PathBuf, collections::HashMap};
use crate::{request::HTTPMethod, routing::match_route_path, JuriPlugin, Request, Response};

pub struct StaticFilePlugin {
    re_urls: Vec<String>,
    dirs: Vec<PathBuf>,
}

impl StaticFilePlugin {
    pub fn new(urls: Vec<String>, dirs: Vec<PathBuf>) -> Self {
        let mut re_urls = vec![];
        for url in urls.iter() {
            re_urls.push(format!("{}{}", url, r"/(.+)"));
        }
        StaticFilePlugin { re_urls, dirs }
    }

    fn find_file_path(&self, url_path: &String) -> Option<PathBuf> {
        for dir in self.dirs.iter() {
            if let Some(path) = dir.to_str() {
                let file_path = PathBuf::from(format!("{}/{}", path, url_path));
                if file_path.exists() {
                    return Some(file_path)
                }
            }
        }
        None
    }
}

impl JuriPlugin for StaticFilePlugin {
    fn request(&self, request: &mut Request) -> Option<Response> {
        if request.method == HTTPMethod::GET {
            for re in self.re_urls.iter() {
                if let Some(params_map) = match_route_path(re.to_string(), vec!["url_path".to_string()], request.path.clone()) {
                    if let Some(url_path) = params_map.get("url_path") {
                        if let Some(file_path) = self.find_file_path(url_path) {
                            return Some(Response {
                                status_code: 200,
                                headers: HashMap::new(),
                                body: crate::ResponseBody::Path(file_path)
                            });
                        }
                    }
                }
            }
        }

        None
    }

    fn response(&self, _request: &Request, _response: &mut Response) {
        
    }
}


#[test]
fn test_file_path() {
    let path = PathBuf::from("/aa/");
    println!("{:#?}", path.to_str());

    let path = path.join("bb/cc.js");
    println!("{:#?}", path.to_str());
}