use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_options::{
    DerivativesTradingOptionsRestApi, rest_api::NewBlockTradeOrderParams,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from env
    let api_key = env::var("API_KEY").context("API_KEY must be set")?;
    let api_secret = env::var("API_SECRET").context("API_SECRET must be set")?;

    // Build REST config
    let rest_conf = ConfigurationRestApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the DerivativesTradingOptions REST API client
    let rest_client = DerivativesTradingOptionsRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        NewBlockTradeOrderParams::builder("liquidity_example".to_string(), [].to_vec()).build()?;

    // Make the API call
    let response = rest_client
        .new_block_trade_order(params)
        .await
        .context("new_block_trade_order request failed")?;

    info!(?response.rate_limits, "new_block_trade_order rate limits");
    let data = response.data().await?;
    info!(?data, "new_block_trade_order data");

    Ok(())
}
