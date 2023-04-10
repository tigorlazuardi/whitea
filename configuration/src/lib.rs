use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::{eyre::Context, Result};
use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};

mod backend_config;
mod shell_string;
mod ytdlp;

pub use backend_config::*;
pub use shell_string::*;
pub use ytdlp::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Configuration {
    pub backend: BackendConfig,
}

impl Configuration {
    /// Dump the configuration as a YAML string.
    pub fn dump_as_yaml(&self) -> Result<String> {
        let s = serde_yaml::to_string(&self)?;
        Ok(s)
    }

    /// Write the configuration to a file in given path.
    pub fn write_config<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let s = self.dump_as_yaml()?;
        std::fs::write(path, s)?;
        Ok(())
    }

    /// Write the configuration to the default path.
    pub fn write_config_to_default_path(&self) -> Result<()> {
        fs::create_dir_all(default_config_path()).wrap_err_with(|| {
            format!(
                "failed to create directory at {}",
                default_config_path().to_string_lossy()
            )
        })?;
        self.write_config(default_config_path().join("whitea.yml"))
    }
}

fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .expect("failed to find user's config dir for current os")
        .join("whitea")
}

static DEFAULT_CONFIG: &str = include_str!("../../whitea.yml");

pub fn get_configuration() -> Result<Configuration, ConfigError> {
    let cfg = Config::builder()
        .add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Yaml))
        .add_source(File::from(default_config_path().join("whitea")).required(false))
        .add_source(File::from(current_dir().unwrap().join("whitea")).required(false))
        .add_source(Environment::with_prefix("whitea").separator("__"))
        .build()?;

    dbg!(cfg.try_deserialize())
}
