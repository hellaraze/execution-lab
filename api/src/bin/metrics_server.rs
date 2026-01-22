use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

async fn metrics() -> impl IntoResponse {
    let _ = observability::init_prometheus();
    let Some(h) = observability::handle() else {
        return (StatusCode::INTERNAL_SERVER_ERROR, String::new());
    };
    (StatusCode::OK, h.render())
}

async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = observability::init_prometheus();

    let app = Router::new()
        .route("/metrics", get(metrics))
        .route("/healthz", get(healthz))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("127.0.0.1:9898").await?;
    println!(
        "metrics server listening on http://{}  (/metrics)",
        listener.local_addr()?
    );

    axum::serve(listener, app).await?;
    Ok(())
}
