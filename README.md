# Jump.rs

A simple Wake-on-LAN (WoL) web server built with Rust and Axum.

## Features

- Wake devices on your network via HTTP API
- Web-based frontend interface
- JSON-based device storage
- Configurable via file or environment variables
- Optional OpenTelemetry tracing support
- Docker support included

## Quick Start

1. Copy the example configuration:
```bash
cp config.toml.example config.toml
```

2. Run the server:
```bash
cargo run
```

3. Access the web interface at `http://localhost:3000`

## Configuration

Configuration can be provided via `config.toml` or environment variables with the `JUMPERS_` prefix.

Example environment variables:
```bash
JUMPERS_SERVER_PORT=8080
JUMPERS_SERVER_LOG_LEVEL=debug
JUMPERS_STORAGE_FILE_PATH=/data/devices.json
```

See `config.toml.example` for all available options.

## Docker

Run with Docker Compose:
```bash
docker-compose up
```

## Development

Build the project:
```bash
cargo build
```

Run with OpenTelemetry tracing:
```bash
cargo run --features otlp
```
