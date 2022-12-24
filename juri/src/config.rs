use crate::web_socket::WSConfig;

pub struct Config {
    pub keep_alive_timeout: u64,
    pub ws: WSConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keep_alive_timeout: 20,
            ws: Default::default(),
        }
    }
}

#[test]
fn test() {
    let config = Config::default();
    assert_eq!(config.keep_alive_timeout, 20);
    assert_eq!(config.ws.keep_alive_timeout, 60);
}
