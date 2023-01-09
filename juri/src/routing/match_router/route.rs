use std::collections::HashMap;

use super::match_route_params;

#[derive(Clone, Debug)]
pub struct MatchRoute {
    pub path: String,
    pub params: Vec<String>,
    pub handler: String,
}

impl MatchRoute {
    pub fn new(path: String, handler: String) -> Self {
        let path_split_list: Vec<&str> = path.split("/:").collect();
        if path_split_list.len() == 1 {
            Self {
                path: format!(r"^{}$", path_split_list[0]),
                params: vec![],
                handler,
            }
        } else {
            let mut path_re = String::from("");
            let mut path_params: Vec<String> = vec![];
            for (index, path) in path_split_list.iter().enumerate() {
                if index == 0 {
                    path_re.push_str(path);
                } else if let Some(index) = path.find('/') {
                    path_params.push(path[..index].to_string());
                    path_re.push_str(format!("{}{}", r"/([^/]*?)", &path[index..]).as_str());
                } else if path.ends_with('+') {
                    path_params.push(path.to_string());
                    path_re.push_str(r"/(.+)");
                    break;
                } else {
                    path_params.push(path.to_string());
                    path_re.push_str(r"/([^/]*?)");
                }
            }
            Self {
                path: format!(r"^{}$", path_re),
                params: path_params,
                handler,
            }
        }
    }
}

impl MatchRoute {
    pub fn match_params(&self, path: String) -> Option<HashMap<String, String>> {
        //TODO 优化
        match_route_params(self.path.clone(), self.params.clone(), path)
    }
}

#[cfg(test)]
mod test {
    use super::MatchRoute;

    #[test]
    fn test_match_route() {
        let match_route = MatchRoute::new(String::from("/aa"), String::from(""));
        assert_eq!(match_route.path, String::from("^/aa$"));

        let match_route = MatchRoute::new(String::from("/aa/:bb"), String::from(""));
        assert_eq!(match_route.path, String::from("^/aa/([^/]*?)$"));

        let match_route = MatchRoute::new(String::from("/aa/:bb/cc"), String::from(""));
        assert_eq!(match_route.path, String::from("^/aa/([^/]*?)/cc$"));

        let match_route = MatchRoute::new(String::from("/aa/:bb+"), String::from(""));
        assert_eq!(match_route.path, String::from("^/aa/(.+)$"));
    }
}
