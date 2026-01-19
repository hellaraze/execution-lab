use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let symbol = args.next().unwrap_or_else(|| "BTCUSDT".to_string());
    let log_path = args.next().unwrap_or_else(|| "/tmp/binance_depth.ndjson".to_string());

    connectors::binance::run_depth_reconstructed(&symbol, &log_path).await
}
