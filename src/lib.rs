pub mod mqtt;
pub mod commands;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub client_id: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 1883,
            client_id: "omnispindle".to_string(),
        }
    }
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn omnispindle(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<mqtt::client::MqttClient>()?;
    Ok(())
} 