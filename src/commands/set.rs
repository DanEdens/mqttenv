use anyhow::Result;
use clap::Parser;
use crate::mqtt::MqttClient;

#[derive(Parser, Debug)]
pub struct SetCommand {
    /// Variable name to set
    #[arg(short, long)]
    name: String,

    /// Value to set
    #[arg(short, long)]
    value: String,

    /// Whether to retain the message
    #[arg(short, long, default_value = "true")]
    retain: bool,
}

impl SetCommand {
    pub async fn execute(&self, client: &MqttClient) -> Result<()> {
        let topic = format!("variables/{}", self.name);
        client.publish(&topic, &self.value, self.retain).await
    }
} 