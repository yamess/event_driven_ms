use crate::redis::RedisStream;
use crate::settings::GlobalConfig;

pub struct AppState {
    pub stream: RedisStream,
    pub config: GlobalConfig,
}

impl AppState {
    pub fn new() -> Self {
        let config = GlobalConfig::new();
        let stream = RedisStream::new(config.redis.clone()).unwrap();
        Self { stream, config }
    }
}