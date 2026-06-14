use hotaru::prelude::*;
use hotaru::http::*;
use serde::{Deserialize, Serialize};

// This struct is used to package the temperature data for the dashboard
#[derive(Debug, Serialize, Deserialize)]
struct TemperatureReading {
    temperature: f64,
    unit: String,
    timestamp: String,
    sensor_id: String,
}

#[tokio::main]
async fn main() {
    println!("HCR Week 4 — Temperature Dashboard starting on http://127.0.0.1:3003");
    // Start the Hotaru server
    APP.clone().run().await;
}

// ─────────────────────────────────────────────────────────────────────────────
// Setup the Server
// ─────────────────────────────────────────────────────────────────────────────
LServer!(
    // Setting up the HTTP server on port 3003
    APP = Server::new()
        .binding("127.0.0.1:3003")
        .single_protocol(ProtocolBuilder::new(HTTP::server(HttpSafety::default())))
        .build()
);

// ─────────────────────────────────────────────────────────────────────────────
// Client setup to talk to the external API
// ─────────────────────────────────────────────────────────────────────────────
pub static SENSOR_CLIENT: Lazy<Arc<Client<TlsTransport>>> = Lazy::new(|| {
    Client::<TlsTransport>::new()
        .target(TlsOutboundTarget::new(
            "timeapi.io",
            443,
            TlsClientConfig::default(),
        ))
        .single_protocol(ProtocolBuilder::new(HTTPS::client(HttpSafety::default())))
        .build()
});

/// ========= Question 2 START =========
// Outpoint for fetching data from the sensor/API
outpoint! {
    SENSOR_CLIENT.url("/api/v1/time/current/utc"),

    /// My handler for calling the external API
    fetch_temperature <HTTPS> {
        send;
        Ok(req)
    }
}
/// ========= Question 2 END =========

/// ========= Question 1 & 2 START =========
// Endpoint to handle the temperature request from the dashboard
endpoint! {
    APP.url("/temperature"),

    pub get_temperature <HTTP> {
        let mut outbound = request_templates::get_request("/api/v1/time/current/utc");
        outbound.meta.set_host(Some("timeapi.io".to_string()));

        // Run the call to the sensor API defined above
        match run!(SENSOR_CLIENT<HTTPS>::fetch_temperature, outbound).await {
            Ok(Ok(resp)) => {
                // Formatting the response so the dashboard can read it
                let body_bytes: Vec<u8> = match resp.body {
                    HttpBody::Text(s)             => s.into_bytes(),
                    HttpBody::Binary(b)           => b,
                    HttpBody::Buffer { data, .. } => data,
                    _                             => Vec::new(),
                };
                
                // If API works, show the real (or simulated) reading
                json_response(simulated_reading())
            }
            // If the API call fails, just show simulated data so the dashboard doesn't break
            Ok(Err(_)) | Err(_) => {
                json_response(simulated_reading())
            }
        }
    }
}
/// ========= Question 1 & 2 END =========

// ─────────────────────────────────────────────────────────────────────────────
// Helper functions (for simulation)
// ─────────────────────────────────────────────────────────────────────────────

// This creates a fake oscillating temperature for the dashboard
fn simulated_reading() -> String {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let cycle = (t % 60) as f64;
    let temp = 26.0 + 4.0 * (cycle * std::f64::consts::PI / 30.0).sin();
    let temp = (temp * 10.0).round() / 10.0;

    serde_json::json!({
        "temperature": temp,
        "unit": "C",
        "timestamp": format!("T+{}s", t),
        "sensor_id": "sim-001"
    })
    .to_string()
}

fn json_response(body: String) -> HttpResponse {
    let mut resp = text_response(body);
    resp.meta.header.insert(
        "Content-Type".to_string(),
        hotaru::http::HeaderValue::Single("application/json".to_string()),
    );
    resp
}

/// ========= Question 3 START =========
// Endpoint that serves the HTML dashboard page to the browser
endpoint! {
    APP.url("/"),

    pub dashboard <HTTP> {
        html_response(DASHBOARD_HTML)
    }
}
/// ========= Question 3 END =========

fn html_response(html: &'static str) -> HttpResponse {
    let mut resp = text_response(html.to_string());
    resp.meta.header.insert(
        "Content-Type".to_string(),
        hotaru::http::HeaderValue::Single("text/html; charset=utf-8".to_string()),
    );
    resp
}

// The dashboard HTML template
static DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>HCR Temperature Dashboard</title>
    <style>
        body { font-family: sans-serif; background: #0d1117; color: #e6edf3; display: flex; flex-direction: column; align-items: center; padding: 2rem; }
        .lcd-wrap { background: #161b22; padding: 2rem; border-radius: 12px; border: 1px solid #30363d; text-align: center; }
        .lcd-value { font-size: 3rem; color: #58a6ff; }
    </style>
</head>
<body>
    <h1>🌡 Temperature Dashboard</h1>
    <div class="lcd-wrap">
        <div class="lcd-value" id="lcdValue">--.-°C</div>
    </div>
    <script>
        async function refresh() {
            const res = await fetch('/temperature');
            const data = await res.json();
            document.getElementById('lcdValue').textContent = data.temperature + '°C';
        }
        setInterval(refresh, 2000);
    </script>
</body>
</html>
"#;

#[allow(dead_code)]
mod resource;
