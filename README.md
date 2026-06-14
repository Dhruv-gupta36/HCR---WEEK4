# HCR - Week 4 Assignment: Temperature Dashboard

This repository contains the solution for the Week 4 assignment, implementing a temperature monitoring dashboard using the **Hotaru framework**. The project utilizes HTTP for the dashboard frontend and MQTT for sensor data communication.

## Assignment Answers

### 1. LCD Display Logic
For letting the LCD display something, we require data flow **in** from the broker.
* *Explanation:* In the MQTT publish-subscribe model, the broker acts as a relay that accepts publishes and fans them out to matching subscribers. The LCD display acts as a subscriber that receives data pushed from the broker.

### 2. Sensor Data Reading
The sensor integration is handled via the `endpoint!` macro using the MQTT protocol. 
* Implementation is located in `src/main.rs`.
* We subscribe to the `sensors/temperature` topic to listen for incoming sensor data.

### 3. Dashboard Display
The dashboard is implemented as an HTTP endpoint that serves an HTML interface.
* The dashboard communicates with the server via the `/temperature` endpoint.
* Real-time updates are handled by the browser-side script that fetches temperature data and updates the LCD component on the UI.

---

## Architecture Overview
This application utilizes the Four Roles defined in the Hotaru framework:
1. **Server:** The HTTP listener serving the dashboard page.
2. **Client:** The component that fetches data from the upstream sensor API.
3. **Broker:** Handles the one-to-many communication for the MQTT bus.
4. **Endpoints/Outpoints:** The internal handlers for incoming and outgoing data.

## Setup and Execution

### Prerequisites
- Rust (latest stable)
- `hotaru` framework (v0.8.2)

### Running the Project
1. Clone this repository.
2. Ensure you have the `hotaru_mqtt` dependency correctly configured in your `Cargo.toml`.
3. Start the server using the terminal:
   ```bash
   cargo run
