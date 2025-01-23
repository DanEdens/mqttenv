use omnispindle::{
    commands::GetCommand,
    mqtt::{MqttClient, MqttConfig},
};
use std::env;
use tokio;

fn get_test_config() -> MqttConfig {
    MqttConfig {
        host: env::var("AWSIP").expect("AWSIP must be set for tests"),
        port: env::var("AWSPORT")
            .expect("AWSPORT must be set for tests")
            .parse()
            .expect("AWSPORT must be a valid port number"),
        client_id: "omnispindle-test-sequence".to_string(),
    }
}

#[tokio::test]
async fn test_nrost_rost_get_sequence() {
    let config = get_test_config();
    let client = MqttClient::new(config.host, config.port, config.client_id)
        .await
        .expect("Failed to create client");

    // Step 1: Clear the topic with nrost
    client.publish("test2", "", true)
        .await
        .expect("Failed to clear topic with nrost");

    // Verify topic is cleared
    client.subscribe("test2")
        .await
        .expect("Failed to subscribe after nrost");

    let result = client.receive_message().await;
    assert!(result.is_err(), "Expected no message after nrost");

    // Step 2: Set new value with rost
    client.publish("test2", "sdfgs", true)
        .await
        .expect("Failed to set value with rost");

    // Step 3: Get and verify value
    let cmd = GetCommand {
        name: "test2".to_string(),
        topic: "test2".to_string(),
    };

    let value = cmd.execute(&client)
        .await
        .expect("Failed to execute get command");
    
    assert_eq!(value, "sdfgs", "Expected value 'sdfgs' from test2 topic after rost");
} 