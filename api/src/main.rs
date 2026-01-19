use connectors::binance::run_depth_reconstructed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_depth_reconstructed("btcusdt", "events_book.log").await
}
