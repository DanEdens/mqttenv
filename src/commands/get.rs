use anyhow::Result;
use clap::Parser;
use crate::mqtt::MqttClient;

#[derive(Parser, Debug)]
pub struct GetCommand {
    /// Variable name to get
    #[arg(short, long)]
    pub name: String,

    /// MQTT topic to get the value from
    #[arg(short, long)]
    pub topic: String,
}

impl GetCommand {
    pub async fn execute(&self, client: &MqttClient) -> Result<String> {
        // Subscribe to topic and get first message
        client.subscribe(&self.topic).await?;
        
        // Wait for a message
        let value = client.receive_message().await?;
        Ok(value)
    }
} 