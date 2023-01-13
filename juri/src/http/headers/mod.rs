use std::{
    collections::{hash_map, HashMap},
    ops::Index,
    slice::{Iter, SliceIndex},
};

#[derive(Clone, Debug)]
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

    pub fn iter(&self) -> Iter<'_, String> {
        self.inner.iter()
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

#[derive(Clone, Default, Debug)]
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

    pub fn iter(&self) -> hash_map::Iter<'_, String, HeaderValues> {
        self.inner.iter()
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
