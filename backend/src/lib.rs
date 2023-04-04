use std::net::SocketAddr;

use axum::{routing::get, Router};
use color_eyre::eyre::{Result, WrapErr};
use configuration::Configuration;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

mod telemetry;

#[tokio::main]
pub async fn start_server(cfg: Configuration) -> Result<()> {
    let subscriber = telemetry::get_subscriber(cfg.backend.log_level.into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        tx.send(()).unwrap();
    });

    let app = Router::new().route("/", get(root)).layer(
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)),
    );

    let addr = format!("{}:{}", cfg.backend.host, cfg.backend.port);
    let socket_addr = addr
        .parse::<SocketAddr>()
        .wrap_err_with(|| format!("unable to parse <host>:<port> address from '{addr}'"))?;
    tracing::info!("server listening on {addr}");
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.unwrap();
            tracing::info!("shutdown signal received, stopping server");
        })
        .await
        .wrap_err("failed to start server")
}

async fn root() -> &'static str {
    "Ok"
}
