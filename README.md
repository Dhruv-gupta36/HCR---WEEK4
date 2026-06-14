# HCR --- WEEK 4: Temperature Dashboard

This project is part of the Hair Cutting Robot (HCR) coursework. It implements a web-based temperature monitoring dashboard using the **Hotaru framework**, utilizing both HTTP and MQTT protocols.

## Project Overview
This application serves as a data bridge for a sensor simulator:
- **Server:** An HTTP server (port 3003) that serves a live temperature dashboard.
- **Client:** Connects to an external sensor API to fetch data.
- **Broker:** Uses MQTT for asynchronous data communication to handle sensor updates.

## Prerequisites
- Rust (latest stable version)
- Hotaru framework
- `tokio` (async runtime)

## How to Run
1. Ensure you have the `hotaru` CLI installed.
2. Navigate to the project directory:
   ```bash
   cd web_hello
