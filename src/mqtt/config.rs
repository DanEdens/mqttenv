use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub client_id: String,
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            host: env::var("AWSIP").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("AWSPORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(1883),
            client_id: env::var("DENA").unwrap_or_else(|_| "omnispindle".to_string()),
        }
    }
}

impl MqttConfig {
    pub fn from_env() -> Self {
        Self::default()
    }

    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = client_id.into();
        self
    }
} 