use crate::cache::main::get_cache_file_path;
use async_std::{
    fs::{File, OpenOptions},
    io::WriteExt,
};
use regex::Regex;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::{SystemTime, UNIX_EPOCH},
};

use super::stream::get_header;

pub struct MultipartFormData {
    pub boundary: String,
    pub form_data_vec: Vec<FormData>,
    pub temp_form_data: Option<FormData>,
}

impl MultipartFormData {
    pub async fn handle_bytes(&mut self, body_bytes: &mut Vec<u8>) {
        let boundary_start_vec = format!("--{}", self.boundary).as_bytes().to_vec();
        let boundary_end_vec = format!("--{}--", self.boundary).as_bytes().to_vec();
        let cache_file_path = get_cache_file_path();

        let mut flag_n = false; // 10
        let mut flag_r = false; // 13
        let mut point_index: usize = 0;
        for (index, byte) in body_bytes.iter().enumerate() {
            if flag_r {
                if *byte == 10 {
                    flag_n = true;
                } else {
                    flag_r = false;
                }
            }

            if *byte == 13 {
                flag_r = true;
            }

            if flag_n && flag_r {
                let mut bytes = body_bytes[point_index..(index - 1)].to_vec();
                if is_vec_equals(&boundary_start_vec, &bytes) {
                    if let Some(temp_form_data) = self.temp_form_data.as_mut() {
                        let file_path =
                            cache_file_path.join(temp_form_data.cache_file_name.clone());
                        let file = OpenOptions::new()
                            .write(true)
                            .open(file_path.clone())
                            .await
                            .unwrap();
                        let file_size = file.metadata().await.unwrap().len();
                        file.set_len(file_size - 2).await.unwrap();

                        self.form_data_vec.push(temp_form_data.clone());
                        self.temp_form_data = None;
                    }

                    let unix = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("get_current_unix_err");
                    self.temp_form_data = Some(FormData {
                        name: "".to_string(),
                        file_name: None,
                        content_type: None,
                        cache_file_name: "".to_owned(),

                        _create_time_seconds: unix.as_secs(),
                        _create_time_nanosecond: unix.subsec_nanos(),
                    });
                    point_index = index + 1;
                } else if is_vec_equals(&boundary_end_vec, &bytes) {
                    if let Some(temp_form_data) = self.temp_form_data.as_mut() {
                        let file_path =
                            cache_file_path.join(temp_form_data.cache_file_name.clone());
                        let file = OpenOptions::new()
                            .write(true)
                            .open(file_path.clone())
                            .await
                            .unwrap();
                        let file_size = file.metadata().await.unwrap().len();
                        file.set_len(file_size - 2).await.unwrap();

                        self.form_data_vec.push(temp_form_data.clone());
                        self.temp_form_data = None;
                    }
                    point_index = index + 1;
                    break;
                } else if let Some(temp_form_data) = self.temp_form_data.as_mut() {
                    if bytes.len() == 0 && temp_form_data.cache_file_name.is_empty() {
                        let mut s = DefaultHasher::new();
                        temp_form_data.hash(&mut s);
                        temp_form_data.cache_file_name = s.finish().to_string();
                    } else if temp_form_data.cache_file_name.is_empty() {
                        let header = String::from_utf8(bytes).unwrap();
                        let (key, value) = get_header(header);
                        if key == "Content-Disposition" {
                            let mut str_split = value.split(";");
                            str_split.next();
                            if let Some(name_str) = str_split.next() {
                                let name_str = name_str.trim();
                                let re = Regex::new("^(.*?)=\"(.*?)\"$").unwrap();
                                let caps = re.captures(&name_str).unwrap();
                                let key = caps
                                    .get(1)
                                    .map_or("".to_string(), |m| m.as_str().trim().to_string());
                                let value = caps
                                    .get(2)
                                    .map_or("".to_string(), |m| m.as_str().trim().to_string());
                                if key == "name" {
                                    temp_form_data.name = value.trim().to_string();
                                }
                            }
                            if let Some(file_name_str) = str_split.next() {
                                let file_name_str = file_name_str.trim();
                                let re = Regex::new("^(.*?)=\"(.*?)\"$").unwrap();
                                let caps = re.captures(&file_name_str).unwrap();
                                let key = caps
                                    .get(1)
                                    .map_or("".to_string(), |m| m.as_str().trim().to_string());
                                let value = caps
                                    .get(2)
                                    .map_or("".to_string(), |m| m.as_str().trim().to_string());
                                if key == "filename" {
                                    temp_form_data.file_name = Some(value.trim().to_string());
                                }
                            }
                        } else if key == "Content-Type" {
                            temp_form_data.content_type = Some(value);
                        }
                    } else {
                        let file_path =
                            cache_file_path.join(temp_form_data.cache_file_name.clone());

                        let mut file;
                        match OpenOptions::new()
                            .append(true)
                            .open(file_path.clone())
                            .await
                        {
                            Ok(temp_file) => {
                                file = temp_file;
                            }
                            Err(_e) => {
                                file = File::create(file_path).await.unwrap();
                            }
                        }

                        bytes.push(13);
                        bytes.push(10);
                        file.write(&bytes).await.unwrap();
                    }
                    point_index = index + 1;
                } else {
                    panic!("Error: Out of the question");
                }
                flag_n = false;
                flag_r = false;
            }
        }
        if point_index < body_bytes.len() {
            if let Some(temp_form_data) = self.temp_form_data.as_mut() {
                let bytes = body_bytes[point_index..].to_vec();

                let file_path = cache_file_path.join(temp_form_data.cache_file_name.clone());
                let mut file;
                match OpenOptions::new()
                    .append(true)
                    .open(file_path.clone())
                    .await
                {
                    Ok(temp_file) => {
                        file = temp_file;
                    }
                    Err(_e) => {
                        file = File::create(file_path).await.unwrap();
                    }
                }

                file.write(&bytes).await.unwrap();
            }
        }
    }
}

fn is_vec_equals<T: std::cmp::PartialEq>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }

    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            return false;
        }
    }

    return true;
}

#[derive(Hash, Clone)]
pub struct FormData {
    pub name: String,
    pub file_name: Option<String>,
    pub content_type: Option<String>,
    pub(crate) _create_time_seconds: u64,
    pub(crate) _create_time_nanosecond: u32,

    pub cache_file_name: String,
}

impl FormData {
    pub fn open(self) -> std::io::Result<std::fs::File> {
        let cache_file_path = get_cache_file_path();
        let file_path = cache_file_path.join(self.cache_file_name.clone());
        std::fs::File::open(file_path)
    }
}
