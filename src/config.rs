pub struct Config {
    pub keep_alive_timeout: u64,
}

impl Config {
    pub fn new() -> Self {
        Config {
            keep_alive_timeout: 20,
        }
    }
}
