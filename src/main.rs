use sunbay_kernel_service::{
    config::Config,
    handlers::{self, health_check, AppState},
    services::{BackendClient, EmvProcessor},
};

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    // 加载配置
    let config = Config::load()?;
    tracing::info!("Configuration loaded");

    // 初始化服务
    let emv_processor = Arc::new(EmvProcessor::new(
        "156".to_string(), // China country code
        "CNY".to_string(), // Chinese Yuan
    ));

    let backend_client = Arc::new(BackendClient::new(config.backend.url.clone()));

    // 创建应用状态
    let state = Arc::new(AppState {
        emv_processor,
        backend_client,
    });

    // 创建路由
    let app = Router::new()
        // 健康检查
        .route("/health", get(health_check))
        // EMV 卡交互 API
        .route("/api/emv/select", post(handlers::select_application))
        .route("/api/emv/read", post(handlers::read_record))
        .route("/api/emv/gpo", post(handlers::get_processing_options))
        // 交易鉴证 API
        .route(
            "/api/transactions/attest",
            post(handlers::attest_transaction),
        )
        .route(
            "/api/transactions/:id/status",
            get(handlers::get_transaction_status),
        )
        // 中间件
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting server on {}", addr);
    tracing::info!("EMV transaction processing and SoftPOS attestation service");
    tracing::info!("Backend URL: {}", config.backend.url);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
