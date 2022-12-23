import subprocess

def set_variable_from_mqtt_topic(name: str, awsip: str, awsport: str, topic: str) -> str:
    """
    Subscribes to the specified MQTT topic and sets the value of the variable with the specified name to the value
    received from the topic.
    """
    # Replace any equals signs in the name with colons
    name = name.replace("=", ":")

    # Get the last character of the name
    last = name[-1]

    # Subscribe to the MQTT topic and get the first message received
    output = subprocess.run(["mosquitto_sub", "-h", awsip, "-p", awsport, "-t", topic, "-C", "1"], capture_output=True)
    get = output.stdout.decode().strip()

    # Set the variable with the specified name to the value received from the MQTT topic
    globals()[name] = get

    return get


