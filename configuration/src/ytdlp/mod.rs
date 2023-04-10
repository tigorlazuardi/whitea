mod network;

pub use network::*;

use serde::{Deserialize, Serialize};

use crate::ShellString;

pub trait YTDLPArguments {
    /// Append arguments to the given vector.
    fn append_arguments(&self, w: &mut Vec<String>);
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct YTDLPConfig {
    pub download_dir: ShellString,
    pub network: Option<YTDLPNetwork>,
}

impl YTDLPArguments for YTDLPConfig {
    fn append_arguments(&self, w: &mut Vec<String>) {
        w.push("--path".into());
        self.download_dir.append_arguments(w);
        if let Some(network) = &self.network {
            network.append_arguments(w);
        }
    }
}

impl YTDLPConfig {
    pub fn build_arguments(&self) -> Vec<String> {
        let mut w = Vec::new();
        self.append_arguments(&mut w);
        w
    }
}
