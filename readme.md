# Rust DDNS Updater

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A high-performance DDNS update tool written in Rust, supporting both IPv4 and IPv6.

## Features

- Support for IPv4 and IPv6 dual-stack updates
- Using Cloudflare API to update DNS records
- Web API interface for manual updates
- Automatic detection of IP changes and DNS record updates
- Multiple configuration methods (environment variables or configuration files)
- Detailed logging
- Lightweight and efficient implementation

## Quick Start

### Installation

1. Make sure Rust and Cargo are installed:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```
   git clone https://github.com/love2004/Rust-DDNS-Updater.git
   cd Rust-DDNS-Updater
   ```

3. Compile the program:
   ```
   cargo build --release
   ```

### Configuration

There are two ways to configure DDNS updates:

#### Using Environment Variables

1. Copy the example environment variable file:
   ```
   cp .env.example .env
   ```

2. Edit the `.env` file and fill in your Cloudflare API information

#### Using Configuration File

1. Copy the example configuration file:
   ```
   cp config/ddns.example.json config/ddns.json
   ```

2. Edit the `config/ddns.json` file and fill in your Cloudflare API information

3. Set in the `.env` file:
   ```
   DDNS_CONFIG_FILE=config/ddns.json
   ```

#### Server Settings

Server settings are stored in the `config/default.toml` file:

```toml
[server]
host = "0.0.0.0"  # Listen address
port = 8080       # Listen port
```

You can modify these settings as needed.

### Running

#### Running as a Service

Run the following command to start the DDNS update service and Web API:

```
cargo run --release
```

Or run the compiled executable directly:

```
./target/release/iploolup
```

#### Running DDNS Update Service Only

```
RUN_MODE=ddns cargo run --release
```

Or:

```
RUST_LOG=info RUN_MODE=ddns ./target/release/iploolup
```

### Using Web API

After starting, the following API endpoints are available:

- Show API information: `GET http://localhost:8080/api/v1/`
- Get current IPv4: `GET http://localhost:8080/api/v1/ip/v4`
- Get current IPv6: `GET http://localhost:8080/api/v1/ip/v6`
- Manually update IPv4 DNS record: `GET http://localhost:8080/api/v1/ddns/update/ipv4`
- Manually update IPv6 DNS record: `GET http://localhost:8080/api/v1/ddns/update/ipv6`
- Update IPv4 DNS record (backwards compatibility): `GET http://localhost:8080/api/v1/ddns/update`

#### API Response Examples

Here are the response formats for each API endpoint:

##### API Root Endpoint (`/api/v1/`)

```json
{
    "status": "success",
    "message": "IP Lookup API",
    "version": "1.0.0",
    "endpoints": {
        "ipv4": "/api/v1/ip/v4",
        "ipv6": "/api/v1/ip/v6",
        "ddns": {
            "ipv4": "/api/v1/ddns/update/ipv4",
            "ipv6": "/api/v1/ddns/update/ipv6"
        }
    }
}
```

##### Get IP (`/api/v1/ip/v4` or `/api/v1/ip/v6`)

```json
{
    "status": "success",
    "data": {
        "ip": "203.0.113.1"  # or IPv6 address
    }
}
```

##### Update DNS Record (`/api/v1/ddns/update/ipv4` or `/api/v1/ddns/update/ipv6`)

```json
{
    "status": "success",
    "message": "DNS record updated",
    "data": {
        "ip": "203.0.113.1",  # or IPv6 address
        "domain": "example.com",
        "ttl": 120,
        "proxied": false
    }
}
```

## Configuration Options

### Environment Variables

| Environment Variable | Description | Default Value |
|----------|------|--------|
| `DDNS_CONFIG_FILE` | Configuration file path | - |
| `CLOUDFLARE_API_TOKEN` | Cloudflare API token | - |
| `CLOUDFLARE_ZONE_ID` | Cloudflare zone ID | - |
| `CLOUDFLARE_RECORD_ID` | IPv4 DNS record ID | - |
| `CLOUDFLARE_RECORD_NAME` | IPv4 DNS record name | - |
| `CLOUDFLARE_API_TOKEN_V6` | IPv6 specific API token (optional) | Same as IPv4 |
| `CLOUDFLARE_ZONE_ID_V6` | IPv6 specific zone ID (optional) | Same as IPv4 |
| `CLOUDFLARE_RECORD_ID_V6` | IPv6 DNS record ID | - |
| `CLOUDFLARE_RECORD_NAME_V6` | IPv6 DNS record name | - |
| `DDNS_UPDATE_INTERVAL` | Update interval (seconds) | 300 |
| `DDNS_UPDATE_INTERVAL_V6` | IPv6 update interval (seconds) | Same as IPv4 |
| `RUN_MODE` | Run mode (web or ddns) | web |
| `RUST_LOG` | Log level (trace, debug, info, warn, error) | info |

### Configuration File Format

Configuration file uses JSON format:

```json
[
  {
    "api_token": "your_cloudflare_api_token",
    "zone_id": "your_cloudflare_zone_id",
    "record_id": "your_cloudflare_record_id",
    "record_name": "example.com",
    "update_interval": 300,
    "ip_type": "ipv4"
  },
  {
    "api_token": "your_cloudflare_api_token_for_ipv6",
    "zone_id": "your_cloudflare_zone_id_for_ipv6",
    "record_id": "your_cloudflare_record_id_for_ipv6",
    "record_name": "example.com",
    "update_interval": 300,
    "ip_type": "ipv6"
  }
]
```

## Security Considerations

- Your Cloudflare API token has permissions to modify DNS records, keep it secure
- It's recommended to use API tokens with limited permissions, only granting necessary access
- If used on public networks, consider adding authentication for the Web API

## Contributing

Issues and Pull Requests are welcome!

## License

MIT License