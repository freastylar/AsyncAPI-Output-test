mod cli;
mod handler;
mod model;
mod policy;
mod utils;
mod warp_server;
use crate::cli::*;
use crate::handler::*;
use async_nats::jetstream::{self};
use clap::Parser;
use log::info;
use std::collections::HashMap;
use utils::*;
mod config;
mod logger;
mod tracing;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    // Load .env file
    config::initialize_env();

    //start warp server
    tokio::spawn(warp_server::server());

    // Initialize logger
    let log_lvl = config::get_env("LOG_LEVEL")
        .unwrap()
        .parse()
        .unwrap_or("INFO".to_string());
    logger::init_logger(&log_lvl);

    // Initialize tracing
    let tracing_enabled: bool = config::get_env("TRACING_ENABLED").unwrap().parse().unwrap();
    if tracing_enabled {
        let _tracer = tracing::init_jaeger_tracer("Rust Codegen Compatibility Test");
    }

    // Connect to NATS server
    let nats_url = config::get_env("SERVER_URL").unwrap();
    info!("Connecting to a NATS server: {}", nats_url);
    let client = async_nats::connect(nats_url).await?;

    // Subscribe to channels

    // Parse CLI arguments
    let args = cli::Args::parse();
    handle_cli(&client, &args.command, &args.message).await?;

    // Listen for messages
    tokio::join!();

    // Shutdown Jaeger Tracer
    if tracing_enabled {
        tracing::shutdown_tracer_provider();
    }
    info!("Shutting down...");
    Ok(())
}
