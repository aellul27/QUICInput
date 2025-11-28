use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct QUICInputConfig {
    pub broadcastip: IpAddr,
    pub port: u16,
    pub max_connections: u8
}

impl Default for QUICInputConfig {
    fn default() -> Self {
        Self {
            broadcastip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 4433,
            max_connections: 1,
        }
    }
}

impl QUICInputConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("port must be greater than 0".into());
        }
        Ok(())
    }
}