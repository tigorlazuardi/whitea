use serde::{Deserialize, Serialize};

use crate::ShellString;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct BackendConfig {
    pub host: String,
    pub port: u16,
    pub log_level: ShellString,
    pub ytdlp: YTDLPConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct YTDLPConfig {
    pub download_dir: ShellString,
}
