use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

mod collector;
mod types;

use collector::StatsCollector;

/// 健康检查端点（无需 Token）
async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "monball-agent",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Token 鉴权：从请求头中取 x-monitor-token 与环境变量比对
async fn stats_handler(
    State(collector): State<Arc<StatsCollector>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let expected_token =
        std::env::var("MONITOR_TOKEN").unwrap_or_else(|_| "monball".to_string());
    let token = headers
        .get("x-monitor-token")
        .and_then(|v| v.to_str().ok());

    if token != Some(expected_token.as_str()) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid token"));
    }

    Ok(Json(collector.get_stats()))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let collector = Arc::new(StatsCollector::new());
    let collector_clone = collector.clone();
    tokio::spawn(async move { collector_clone.start_background_task().await });

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/v1/stats", get(stats_handler))
        .with_state(collector)
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "26666".to_string());
    let addr = format!("0.0.0.0:{}", port);
    info!("MonBall Agent v{} listening on {}", env!("CARGO_PKG_VERSION"), addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
