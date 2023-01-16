use chrono::prelude::*;
use std::time::{Duration, SystemTime};

#[derive(Default)]
pub enum SameSite {
    Strict,
    #[default]
    Lax,
    None,
}

impl ToString for SameSite {
    fn to_string(&self) -> String {
        match self {
            SameSite::Strict => "Strict".into(),
            SameSite::Lax => "Lax".into(),
            SameSite::None => "None".into(),
        }
    }
}

pub struct Cookie {
    name: String,
    value: String,
    expires: Option<SystemTime>,
    max_age: Option<Duration>,
    secure: bool,
    http_only: bool,
    domain: Option<String>,
    path: Option<String>,
    same_site: Option<SameSite>,
}

impl Cookie {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            expires: Default::default(),
            max_age: Default::default(),
            secure: Default::default(),
            http_only: Default::default(),
            domain: Default::default(),
            path: Default::default(),
            same_site: Default::default(),
        }
    }

    pub fn set_expires(&mut self, expires: Option<SystemTime>) -> &mut Self {
        self.expires = expires;
        self
    }

    pub fn set_max_age(&mut self, max_age: Option<Duration>) -> &mut Self {
        self.max_age = max_age;
        self
    }

    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = secure;
        self
    }

    pub fn set_http_only(&mut self, http_only: bool) -> &mut Self {
        self.http_only = http_only;
        self
    }

    pub fn set_domain(&mut self, domain: Option<String>) -> &mut Self {
        self.domain = domain;
        self
    }

    pub fn set_path(&mut self, path: Option<String>) -> &mut Self {
        self.path = path;
        self
    }

    pub fn set_same_site(&mut self, same_site: Option<SameSite>) -> &mut Self {
        self.same_site = same_site;
        self
    }
}

impl ToString for Cookie {
    fn to_string(&self) -> String {
        let mut s = format!("{}={}", self.name, self.value);
        if let Some(expires) = &self.expires {
            let expires: DateTime<Utc> = (*expires).into();
            let expires = expires.format("%d %b %Y %H:%M:%S GMT").to_string();
            s.push_str(&format!("; expires=Wed, {}", expires));
        }
        if let Some(max_age) = &self.max_age {
            s.push_str(&format!("; max-age={}", max_age.as_secs_f64()));
        }
        if self.secure {
            s.push_str("; Secure");
        }
        if self.http_only {
            s.push_str("; HttpOnly");
        }
        if let Some(domain) = &self.domain {
            s.push_str(&format!("; Domain={}", domain));
        }
        if let Some(path) = &self.path {
            s.push_str(&format!("; Path={}", path));
        }
        if let Some(same_site) = &self.same_site {
            s.push_str(&format!("; SameSite={}", same_site.to_string()));
        }
        s
    }
}

#[test]
fn test_cookie() {
    let mut cookie = Cookie::new("hi", "hello");
    cookie.set_expires(Some(SystemTime::now()));
    println!("{}", cookie.to_string());
}

#[test]
fn text_time_format() {
    let expires = SystemTime::now();
    let expires: DateTime<Utc> = expires.clone().into();
    // let expires = expires.with_hour(23).unwrap();
    let expires = expires.format("%d %b %Y %H:%M:%S %I GMT").to_string();
    println!("{expires}");
}
