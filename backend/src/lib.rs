use std::net::SocketAddr;

use axum::{routing::get, Router};
use color_eyre::eyre::{Result, WrapErr};
use configuration::Configuration;

mod telemetry;

#[tokio::main]
pub async fn start_server(cfg: Configuration) -> Result<()> {
    let subscriber = telemetry::get_subscriber(cfg.backend.log_level.into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let app = Router::new().route("/", get(root));

    let addr = format!("{}:{}", cfg.backend.host, cfg.backend.port);
    let socket_addr = addr
        .parse::<SocketAddr>()
        .wrap_err_with(|| format!("unable to parse address from '{addr}'"))?;
    tracing::info!("server listening on {addr}");
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .wrap_err("failed to start server")
}

async fn root() -> &'static str {
    "Hello, World!"
}
