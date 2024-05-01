use serde::{Deserialize, Serialize};

// Logger Config
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoggerConfig {
    pub file_path: String,
    pub file_size: u64,
    pub file_count: u32,
}
impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig {
            file_path: "logs/crawler.log".to_string(),
            file_size: 1024 * 1024 * 10,
            file_count: 5,
        }
    }
}
impl LoggerConfig {
    pub fn new() -> LoggerConfig {
        envy::from_env::<LoggerConfig>().unwrap_or_default()
    }
}

// Server Config
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        log::info!("No server configuration provided, using default values");
        ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        envy::from_env::<ServerConfig>().unwrap_or_default()
    }
}

// Redis Config
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RedisConfig {
    pub redis_connection_string: String,
}
impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig {
            redis_connection_string: "redis://0.0.0.0:6379".to_string(),
        }
    }
}
impl RedisConfig {
    pub fn new() -> RedisConfig {
        envy::from_env::<RedisConfig>().unwrap_or_default()
    }
}


// Config
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GlobalConfig {
    pub server: ServerConfig,
    pub logger: LoggerConfig,
    pub redis: RedisConfig,
}

impl GlobalConfig {
    pub fn new() -> Self {
        let server = ServerConfig::new();
        let logger = LoggerConfig::new();
        let redis = RedisConfig::new();
        Self {
            server,
            logger,
            redis,
        }
    }
}
