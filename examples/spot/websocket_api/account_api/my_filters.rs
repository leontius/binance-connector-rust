use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::spot::{SpotWsApi, websocket_api::MyFiltersParams};

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from env
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the environment");
    let api_secret = env::var("API_SECRET").expect("API_SECRET must be set in the environment");

    // Build WebSocket API config
    let ws_api_conf = ConfigurationWebsocketApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the Spot WebSocket API client
    let ws_api_client = SpotWsApi::production(ws_api_conf);

    // Connect to WebSocket
    let connection = ws_api_client
        .connect()
        .await
        .context("Failed to connect to WebSocket API")?;

    // Setup the WS API parameters
    let params = MyFiltersParams::builder("BNBUSDT".to_string()).build()?;

    // Make the WS API call
    let response = connection
        .my_filters(params)
        .await
        .context("my_filters request failed")?;

    info!(?response.rate_limits, "my_filters rate limits");
    let data = response.data()?;
    info!(?data, "my_filters data");

    // Cleanly disconnect
    connection
        .disconnect()
        .await
        .context("Failed to disconnect WebSocket client")?;

    Ok(())
}
