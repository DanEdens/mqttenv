use anyhow::Result;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::{sync::Arc, time::Duration};
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, error, info};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python", pyclass)]
pub struct MqttClient {
    client: AsyncClient,
    receiver: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl MqttClient {
    pub async fn new(host: String, port: u16, client_id: String) -> Result<Self> {
        let mut mqttopts = MqttOptions::new(client_id, host, port);
        mqttopts.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(mqttopts, 10);
        let (tx, rx) = mpsc::channel(100);
        let receiver = Arc::new(Mutex::new(rx));
        
        // Spawn event loop handler
        let tx = Arc::new(Mutex::new(tx));
        tokio::spawn(async move {
            Self::handle_events(eventloop, tx).await;
        });

        Ok(Self { client, receiver })
    }

    async fn handle_events(mut eventloop: rumqttc::EventLoop, tx: Arc<Mutex<mpsc::Sender<String>>>) {
        loop {
            match eventloop.poll().await {
                Ok(notification) => {
                    debug!("Event: {:?}", notification);
                    if let Event::Incoming(Packet::Publish(publish)) = notification {
                        if let Ok(payload) = String::from_utf8(publish.payload.to_vec()) {
                            if let Ok(tx) = tx.try_lock() {
                                if let Err(e) = tx.try_send(payload) {
                                    error!("Failed to send message: {}", e);
                                }
                            }
                        }
                    }
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

    pub async fn receive_message(&self) -> Result<String> {
        let mut rx = self.receiver.lock().await;
        Ok(rx.recv().await.ok_or_else(|| anyhow::anyhow!("No message received"))?)
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

    fn py_receive_message(&self) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new()?;
        match rt.block_on(async { self.receive_message().await }) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
} 