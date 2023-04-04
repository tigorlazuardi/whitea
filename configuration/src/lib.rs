use serde::{Deserialize, Serialize};

mod backend_config;
mod shell_string;

pub use backend_config::*;
pub use shell_string::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Configuration {
    pub backend: BackendConfig,
}

static DEFAULT_CONFIG: &str = include_str!("../../default_config.yml");

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let cfg = config::Config::builder()
        .add_source(config::File::from_str(
            DEFAULT_CONFIG,
            config::FileFormat::Yaml,
        ))
        .build()?;

    cfg.try_deserialize()
}
