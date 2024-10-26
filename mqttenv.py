import os
import subprocess
import json
import logging
import threading

class MQTTEnv:
    def __init__(self, awsip=None, awsport=None, dena=None):
        self.awsip = awsip or os.getenv("AWSIP")
        self.awsport = awsport or os.getenv("AWSPORT")
        self.dena = dena or os.getenv("DENA")
        self.callbacks = {}
        logging.basicConfig(level=logging.INFO)

    def set_variable(self, name: str, value: str, retain: bool = True):
        topic = f"variables/{name}"
        self.publish_message(topic, value, retain)
        logging.info(f"Set variable '{name}' to '{value}'")

    def get_variable(self, name: str) -> str:
        topic = f"variables/{name}"
        return self.set_variable_from_mqtt_topic(name, topic)

    def set_variable_from_mqtt_topic(self, name: str, topic: str) -> str:
        name = name.replace("=", ":")
        output = subprocess.run(
            ["mosquitto_sub", "-h", self.awsip, "-p", self.awsport, "-t", topic, "-C", "1"],
            capture_output=True
        )
        get = output.stdout.decode().strip()
        globals()[name] = get
        logging.info(f"Retrieved variable '{name}' with value '{get}' from topic '{topic}'")
        return get

    def subscribe_to_mqtt_topic(self, topic: str, path: str) -> str:
        topic = topic.replace("=", ":")
        last = topic[-1]
        subprocess.run(
            ["mosquitto_sub", "-h", self.awsip, "-p", self.awsport, "-t", f"commands/{topic}", "-C", "1"],
            stdout=open(f"{path}/{last}.txt", "w")
        )
        return f"{path}/{last}.txt"

    def publish_message(self, topic: str, message: str, retain: bool = False):
        retain_flag = "-r" if retain else ""
        subprocess.run(
            ["mosquitto_pub", retain_flag, "-h", self.awsip, "-p", self.awsport, "-i", self.dena, "-t", topic, "-m", message]
        )
        logging.info(f"Published message to topic '{topic}': {message}")

    def clear_retained_message(self, topic: str):
        subprocess.run(
            ["mosquitto_pub", "-q", "0", "-h", self.awsip, "-p", self.awsport, "-i", self.dena, "-t", topic, "-r", "-n"]
        )
        logging.info(f"Cleared retained message for topic '{topic}'")

    def publish_file(self, topic: str, file_path: str, retain: bool = False):
        retain_flag = "-r" if retain else ""
        subprocess.run(
            ["mosquitto_pub", retain_flag, "-h", self.awsip, "-p", self.awsport, "-i", self.dena, "-t", topic, "-f", file_path]
        )
        logging.info(f"Published file '{file_path}' to topic '{topic}'")

    def get_ip_and_publish(self, topic: str):
        ip = subprocess.check_output("ipconfig | findstr /R /C:\"IPv4 Address\"", shell=True).decode().strip()
        ip = ip.split(":")[-1].strip()
        self.publish_message(topic, ip, retain=True)
        logging.info(f"Published IP address '{ip}' to topic '{topic}'")

    def register_callback(self, topic: str, callback):
        """
        Register a callback function to be executed when a message is received on the specified topic.
        """
        self.callbacks[topic] = callback
        logging.info(f"Registered callback for topic '{topic}'")

    def listen(self):
        """
        Start listening for messages on all registered topics and execute the corresponding callbacks.
        """
        def listen_to_topic(topic, callback):
            process = subprocess.Popen(
                ["mosquitto_sub", "-h", self.awsip, "-p", self.awsport, "-t", topic],
                stdout=subprocess.PIPE
            )
            for line in iter(process.stdout.readline, b''):
                message = line.decode().strip()
                logging.info(f"Received message on topic '{topic}': {message}")
                callback(message)

        threads = []
        for topic, callback in self.callbacks.items():
            thread = threading.Thread(target=listen_to_topic, args=(topic, callback))
            thread.start()
            threads.append(thread)

        for thread in threads:
            thread.join()

# Example usage
if __name__ == "__main__":
    def example_callback(message):
        print(f"Callback received message: {message}")

    mqtt_env = MQTTEnv()
    mqtt_env.register_callback("example/topic", example_callback)
    mqtt_env.listen()
