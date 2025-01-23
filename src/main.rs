use anyhow::Result;
use clap::Parser;
use omnispindle::{
    commands::{GetCommand, SetCommand, SubscribeCommand},
    mqtt::{MqttClient, MqttConfig},
};
use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Command {
    Get(GetCommand),
    Set(SetCommand),
    Subscribe(SubscribeCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let command = Command::parse();

    // Create MQTT client
    let config = MqttConfig::from_env();
    let client = MqttClient::new(config.host, config.port, config.client_id).await?;

    // Execute command
    match command {
        Command::Get(cmd) => {
            let value = cmd.execute(&client).await?;
            info!("Got value: {}", value);
        }
        Command::Set(cmd) => {
            cmd.execute(&client).await?;
            info!("Value set successfully");
        }
        Command::Subscribe(cmd) => {
            let path = cmd.execute(&client).await?;
            info!("Subscribed, output will be written to: {}", path.display());
        }
    }

    Ok(())
} 