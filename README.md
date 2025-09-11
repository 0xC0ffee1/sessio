# Sessio - P2P Shells

## Table of contents

- [About](#about)
- [Features](#features)
- [Install & Usage](#install--usage)

## About

Sessio is a remote device management tool that uses UDP hole punching to establish peer‑to‑peer SSH sessions between devices behind firewalls and NATs.


## Features

- **SSH protocol** Uses the SSH 2.0 protocol (russh)
- **QUIC transport:** Seamless sessions even across network changes; UDP simplifies reliable P2P establishment.
- **UDP hole punching:** Create direct device‑to‑device connections without opening inbound ports.
- **Coordination server:** Public rendezvous server used only to exchange addressing info and coordinate sessions.
- **IPv6 first, IPv4 supported:** Works best with IPv6; most IPv4 NAT types are supported (not Symmetric NAT).
- **Multiplexing:** Multiple SSH sessions share a single QUIC connection via bidirectional streams (no HOL blocking).
- **SFTP and local port‑forwarding:** Minimal SFTP and local forwarding supported.
- **API/CLI/GUI friendly:** gRPC interface exposed for building UIs and integrations.

## Install & Usage
> Sessio is in beta — not recommended for production use.

Sessio requires a Coordinator server. The Coordinator provides rendezvous, device identity, and policy — it does not relay SSH traffic. Devices establish direct QUIC connections via UDP hole‑punching. The Coordinator and its clients follow a zero‑trust model: devices are not trusted until explicitly authorized with your passkey on the web UI.

### Prerequisites
- A running Coordinator with a public URL (HTTPS recommended), or
- Self‑host the Coordinator using Docker Compose.

### Option A — Use an existing Coordinator
- Obtain the Coordinator URL (e.g., `https://your-coord.example.com`).
- You will sign in with a passkey (WebAuthn) on the Coordinator’s web UI during setup.

### Option B — Self‑host the Coordinator (Docker)
- Use the Compose file at `rust/coordinator/docker-compose.yml` as a starting point.
- Create `rust/coordinator/config/coordinator.env` with the required environment variables for Postgres and the Coordinator.
- Provide TLS termination in front of the stack (e.g., via the included Nginx config) or run the Coordinator in HTTPS mode with valid certificates.
- Bring the stack up:
  - `docker compose -f rust/coordinator/docker-compose.yml up -d`
- The web UI runs behind Nginx; expose the frontend port you configured (default 8000).

### Setup Flow
- **Create account:** Open the Coordinator web UI and register/sign in using your passkey (WebAuthn).
- **Add device:** In the UI, create an install key for each device you want to connect.
- **Install on device:** Install the Sessio components (client daemon and/or server) and provide:
  - The Coordinator URL
  - The install key for that device
- **Authorize device:** In the Coordinator web UI, select the new device and “Sign” it using your passkey. This authorizes the device for use under your account.
- **Use Sessio:**
  - Devices will periodically heartbeat to the Coordinator.
  - Initiate P2P SSH sessions via Sessio (client CLI or integrations). Traffic flows directly between peers; the Coordinator never relays user traffic.

### Security Model (Zero Trust)
- The Coordinator stores device metadata and enforces authorization but is not trusted to act for devices.
- Each device must be explicitly authorized (signed) by the account owner’s passkey before it can participate.
- Device authorization is distrubuted as signed keys (e.g., for `authorized_keys` and known hosts) and is verified locally on each device.

## CLI Usage

Once your Coordinator is running and devices are authorized (signed with your passkey in the web UI), you can use the Sessio CLI on your client machine.

- Open an interactive shell to a device
  - `sessio shell <device_id>`
  - Starts an ephemeral session and attaches your terminal.

- Start local port forwarding to a remote service through a device (beta)
  - `sessio forward start <device_id> <local_port:remote_host:remote_port>`
  - Example: `sessio forward start laptop-client 8080:localhost:80`
  - Stop with Ctrl+C in the terminal where forwarding is running.
  
- Check status and available devices
  - `sessio status`
  - Shows client/server service status and lists devices with their last seen time and categories.


