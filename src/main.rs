use std::process;
use tracing::{error, info};

use http2_web_filter::config;

fn main() {
    // Initialize logging first
    init_logging();

    info!("HTTP/2 Web Filter starting...");

    // Load configuration
    let config = match config::load_config() {
        Ok(cfg) => {
            info!("Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    info!("Configuration loaded");
    info!(
        "Proxy listening on {}:{}",
        config.proxy.host, config.proxy.port
    );
    info!("Max connections: {}", config.proxy.max_connections);
    info!("Connection timeout: {}s", config.proxy.timeout_seconds);
    info!("CA certificate path: {}", config.ca.cert_path);
    info!("Audit output: {}", config.audit.output);
    info!("Logging level: {}", config.logging.level);

    // TODO: Start proxy server (M1)
    info!("Server initialization complete (proxy implementation in M1)");

    // Keep main thread alive
    loop {
        std::thread::park();
    }
}

fn init_logging() {
    use tracing_subscriber::{fmt, EnvFilter};

    let _filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt::init();

    // Initialize with JSON formatting if needed
    // For now using default format
}
