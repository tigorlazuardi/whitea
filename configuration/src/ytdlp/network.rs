use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use url::Url;

use super::YTDLPArguments;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum YTDLPProtocol {
    Auto,
    IPV4,
    IPV6,
}

impl YTDLPArguments for YTDLPProtocol {
    fn append_arguments(&self, w: &mut Vec<String>) {
        match self {
            YTDLPProtocol::Auto => (),
            YTDLPProtocol::IPV4 => w.push("-4".into()),
            YTDLPProtocol::IPV6 => w.push("-6".into()),
        }
    }
}

impl Default for YTDLPProtocol {
    fn default() -> Self {
        YTDLPProtocol::Auto
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct YTDLPNetwork {
    pub proxy: Option<Url>,
    pub socket_timeout: Option<u64>,
    pub socket_address: Option<SocketAddr>,
    #[serde(default)]
    pub protocol: YTDLPProtocol,
}

impl YTDLPArguments for YTDLPNetwork {
    fn append_arguments(&self, w: &mut Vec<String>) {
        if let Some(proxy) = &self.proxy {
            w.push("--proxy".into());
            w.push(proxy.to_string());
        }
        if let Some(socket_timeout) = &self.socket_timeout {
            w.push("--socket-timeout".into());
            w.push(socket_timeout.to_string());
        }
        if let Some(socket_address) = &self.socket_address {
            w.push("--socket-address".into());
            w.push(socket_address.to_string());
        }
        self.protocol.append_arguments(w);
    }
}
