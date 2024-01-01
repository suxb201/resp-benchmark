use crate::case::Ping;
use crate::case::Set;

use crate::resp;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(flatten)]
    pub redis_config: resp::RedisConfig,
    pub memory_field: String,
    pub threads: u64,

    pub ping: Option<Vec<Ping>>,
    pub set: Option<Vec<Set>>,
}

impl Config {
    pub fn from_file(path: &str) -> Config {
        let s = std::fs::read_to_string(path).unwrap();
        toml::from_str(&s).unwrap_or_else(|err| {
            println!("Error parsing config file: {}", err);
            std::process::exit(1);
        })
    }

    pub fn clone_redis_config(&self) -> resp::RedisConfig {
        self.redis_config.clone()
    }
}
