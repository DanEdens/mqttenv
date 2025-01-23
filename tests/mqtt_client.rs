use omnispindle::mqtt::{MqttClient, MqttConfig};
use std::env;
use tokio;

fn get_test_config() -> MqttConfig {
    MqttConfig {
        host: env::var("AWSIP").expect("AWSIP must be set for tests"),
        port: env::var("AWSPORT")
            .expect("AWSPORT must be set for tests")
            .parse()
            .expect("AWSPORT must be a valid port number"),
        client_id: "omnispindle-test".to_string(),
    }
}

#[tokio::test]
async fn test_client_connection() {
    let config = get_test_config();
    let client = MqttClient::new(config.host, config.port, config.client_id)
        .await
        .expect("Failed to create client");

    // Test publish
    client
        .publish("test/topic", "test message", false)
        .await
        .expect("Failed to publish message");

    // Test subscribe
    client
        .subscribe("test/topic")
        .await
        .expect("Failed to subscribe to topic");
}

#[tokio::test]
async fn test_retained_messages() {
    let config = get_test_config();
    let client = MqttClient::new(config.host.clone(), config.port, "test-retained".to_string())
        .await
        .expect("Failed to create client");

    let test_topic = "test/retained";
    let test_message = "retained message";

    // Publish retained message
    client
        .publish(test_topic, test_message, true)
        .await
        .expect("Failed to publish retained message");

    // Subscribe to verify
    client
        .subscribe(test_topic)
        .await
        .expect("Failed to subscribe to topic");

    // Clean up - remove retained message
    client
        .publish(test_topic, "", true)
        .await
        .expect("Failed to clean up retained message");
} 