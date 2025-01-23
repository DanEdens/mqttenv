# Omnispindle 🌀

**Your Information Conductor for the Madness_Interactive Ecosystem**

Welcome to **Omnispindle**

## Installation 🔧

### From Source

```bash
# Clone the repository
git clone https://github.com/DanEdens/Omnispindle
cd Omnispindle

# Build Rust binary
cargo build --release

# Install Python package (optional)
pip install .

# Add CLI command to PATH (Linux/Mac)
echo 'export PATH="$PATH:$(pwd)/target/release"' >> ~/.bashrc
source ~/.bashrc
```

```powershell
REM Clone the repository
git clone https://github.com/DanEdens/Omnispindle
cd Omnispindle

REM Build Rust binary
cargo build --release

REM Install Python package (optional)
pip install .

REM Add CLI command to PATH (Windows)
setx PATH "%PATH%;%cd%\target\release"
```

### Environment Variables

- `AWSIP`: MQTT broker host (default: "localhost")
- `AWSPORT`: MQTT broker port (default: 1883)
- `DENA`: Client ID (default: "omnispindle")

## Usage 🚀

### Command Line Interface

```bash
# Get a variable
omnispindle get --name <name> --topic <topic>

# Set a variable
omnispindle set --name <name> --value <value> [--retain]

# Subscribe to a topic
omnispindle subscribe --topic <topic> --path <output_path>
```

### Python API

```python
from omnispindle import MqttClient

# Create client
client = MqttClient(host="localhost", port=1883, client_id="my-client")

# Publish message
client.publish("my/topic", "Hello, World!", retain=True)

# Subscribe to topic
client.subscribe("my/topic")
```

## Development 🛠️

### Building

```bash
# Build Rust binary
cargo build

# Run tests
cargo test

# Build Python package
maturin develop
```

### Project Structure

```
Omnispindle/
├── src/                    # Rust source code
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Core functionality
│   ├── mqtt/              # MQTT implementation
│   └── commands/          # CLI commands
├── python/                # Python bindings
└── README.md
```

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request.

## License 📄

This project is licensed under the MIT License - see the LICENSE file for details.
