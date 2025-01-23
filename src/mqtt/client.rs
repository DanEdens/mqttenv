use anyhow::Result;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use tracing::{debug, error, info};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python", pyclass)]
pub struct MqttClient {
    client: AsyncClient,
}

impl MqttClient {
    pub async fn new(host: String, port: u16, client_id: String) -> Result<Self> {
        let mut mqttopts = MqttOptions::new(client_id, host, port);
        mqttopts.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(mqttopts, 10);
        
        // Spawn event loop handler
        tokio::spawn(async move {
            Self::handle_events(eventloop).await;
        });

        Ok(Self { client })
    }

    async fn handle_events(mut eventloop: rumqttc::EventLoop) {
        loop {
            match eventloop.poll().await {
                Ok(notification) => {
                    debug!("Event: {:?}", notification);
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                }
            }
        }
    }

    pub async fn publish(&self, topic: &str, payload: &str, retain: bool) -> Result<()> {
        self.client
            .publish(topic, QoS::AtLeastOnce, retain, payload.as_bytes())
            .await?;
        info!("Published to {}: {}", topic, payload);
        Ok(())
    }

    pub async fn subscribe(&self, topic: &str) -> Result<()> {
        self.client.subscribe(topic, QoS::AtLeastOnce).await?;
        info!("Subscribed to {}", topic);
        Ok(())
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl MqttClient {
    #[new]
    fn py_new(host: String, port: u16, client_id: String) -> PyResult<Self> {
        let rt = tokio::runtime::Runtime::new()?;
        match rt.block_on(async { Self::new(host, port, client_id).await }) {
            Ok(client) => Ok(client),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }

    fn py_publish(&self, topic: String, payload: String, retain: bool) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()?;
        match rt.block_on(async { self.publish(&topic, &payload, retain).await }) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }

    fn py_subscribe(&self, topic: String) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()?;
        match rt.block_on(async { self.subscribe(&topic).await }) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn get_test_config() -> (String, u16, String) {
        (
            env::var("AWSIP").expect("AWSIP must be set for tests"),
            env::var("AWSPORT")
                .expect("AWSPORT must be set for tests")
                .parse()
                .expect("AWSPORT must be a valid port number"),
            "omnispindle-unit-test".to_string(),
        )
    }

    #[tokio::test]
    async fn test_new_client() {
        let (host, port, client_id) = get_test_config();
        let client = MqttClient::new(host, port, client_id).await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_publish_subscribe() {
        let (host, port, client_id) = get_test_config();
        let client = MqttClient::new(host, port, client_id)
            .await
            .expect("Failed to create client");

        // Test publish
        let result = client.publish("test/unit", "test message", false).await;
        assert!(result.is_ok());

        // Test subscribe
        let result = client.subscribe("test/unit").await;
        assert!(result.is_ok());
    }
} 