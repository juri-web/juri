#[derive(Clone)]
pub struct WSConfig {
    pub keep_alive_timeout: u64,
}

impl Default for WSConfig {
    fn default() -> Self {
        Self {
            keep_alive_timeout: 60,
        }
    }
}
