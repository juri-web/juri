use std::{collections::HashMap, ops::Index, slice::SliceIndex};

#[derive(Clone)]
pub struct HeaderValues {
    inner: Vec<String>,
}

impl HeaderValues {
    pub fn append(&mut self, value: String) {
        self.inner.push(value);
    }

    pub fn last(&self) -> Option<&String> {
        self.inner.last()
    }
}

impl<I: SliceIndex<[String]>> Index<I> for HeaderValues {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.inner[index]
    }
}

impl From<Vec<String>> for HeaderValues {
    fn from(inner: Vec<String>) -> Self {
        HeaderValues { inner }
    }
}

#[derive(Clone, Default)]
pub struct Headers {
    inner: HashMap<String, HeaderValues>,
}

impl Headers {
    pub fn get(&self, key: &str) -> Option<&HeaderValues> {
        self.inner.get(&key.to_ascii_lowercase())
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        if let Some(values) = self.inner.get_mut(key) {
            values.append(value.to_string());
        } else {
            self.inner.insert(
                key.to_ascii_lowercase(),
                HeaderValues::from(vec![value.to_string()]),
            );
        }
    }
}

#[test]
fn test_header() {
    let mut headers = Headers::default();

    headers.insert("hi", "hello");

    if let Some(value) = headers.get("hi") {
        let _1 = value[0].clone();
        assert_eq!(_1, String::from("hello"));
        let _last = value.last();
        assert_eq!(_last, Some(&String::from("hello")));
    }
}
