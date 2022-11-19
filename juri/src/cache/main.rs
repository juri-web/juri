use std::{env, fs, path::PathBuf};

pub fn get_cache_file_path() -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    current_dir.join(".juri").join("cache").join("file")
}

pub fn init_cache() {
    let current_dir = env::current_dir().unwrap();

    let juri = current_dir.join(".juri");
    let juri_path = juri.to_str().unwrap();
    fs::create_dir_all(juri_path).unwrap();

    let cache = juri.join("cache");
    let cache_path = cache.to_str().unwrap();
    fs::create_dir_all(cache_path).unwrap();

    let file = cache.join("file");
    let file_path = file.to_str().unwrap();
    fs::create_dir_all(file_path).unwrap();
}
