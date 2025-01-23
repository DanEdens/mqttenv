use omnispindle::{
    commands::{GetCommand, SetCommand, SubscribeCommand},
    mqtt::{MqttClient, MqttConfig},
};
use std::{env, path::PathBuf};
use tokio;

fn get_test_config() -> MqttConfig {
    MqttConfig {
        host: env::var("AWSIP").expect("AWSIP must be set for tests"),
        port: env::var("AWSPORT")
            .expect("AWSPORT must be set for tests")
            .parse()
            .expect("AWSPORT must be a valid port number"),
        client_id: "omnispindle-test-commands".to_string(),
    }
}

#[tokio::test]
async fn test_set_command() {
    let config = get_test_config();
    let client = MqttClient::new(config.host, config.port, config.client_id)
        .await
        .expect("Failed to create client");

    let cmd = SetCommand {
        name: "test_var".to_string(),
        value: "test_value".to_string(),
        retain: true,
    };

    cmd.execute(&client)
        .await
        .expect("Failed to execute set command");
}

#[tokio::test]
async fn test_get_command() {
    let config = get_test_config();
    let client = MqttClient::new(config.host, config.port, config.client_id)
        .await
        .expect("Failed to create client");

    let cmd = GetCommand {
        name: "test_var".to_string(),
        topic: "variables/test_var".to_string(),
    };

    let _value = cmd.execute(&client)
        .await
        .expect("Failed to execute get command");
}

#[tokio::test]
async fn test_subscribe_command() {
    let config = get_test_config();
    let client = MqttClient::new(config.host, config.port, config.client_id)
        .await
        .expect("Failed to create client");

    let cmd = SubscribeCommand {
        topic: "test/topic".to_string(),
        path: PathBuf::from("test_output"),
    };

    let _path = cmd.execute(&client)
        .await
        .expect("Failed to execute subscribe command");
} 