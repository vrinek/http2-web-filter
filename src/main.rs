use std::env;
use std::process;
use tracing::{error, info};

use http2_web_filter::config;
use http2_web_filter::config::types::{LogFormat, LoggingConfig};

#[tokio::main]
async fn main() {
    // Check for --validate-config flag before initializing logging
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--validate-config".to_string()) {
        validate_config_only().await;
        return;
    }

    // Load configuration FIRST (before logging initialization)
    let config = match config::load_config().await {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

    // Initialize logging with config settings
    init_logging(&config.logging);

    info!("HTTP/2 Web Filter starting...");
    info!("Configuration loaded successfully");
    info!(
        "Proxy listening on {}:{}",
        config.proxy.host, config.proxy.port
    );
    info!("Max connections: {}", config.proxy.max_connections);
    info!("Connection timeout: {}s", config.proxy.timeout_seconds);
    info!("CA certificate path: {}", config.ca.cert_path);
    info!("Audit output: {:?}", config.audit.output);
    info!("Logging level: {:?}", config.logging.level);
    info!("Logging format: {:?}", config.logging.format);

    // TODO: Start proxy server (M1)
    info!("Server initialization complete (proxy implementation in M1)");

    // Wait for shutdown signals
    if let Err(e) = wait_for_shutdown().await {
        error!("Error during shutdown signal handling: {}", e);
    }

    info!("Shutdown complete");
}

/// Validate configuration without starting the service
/// Exits with status 0 if valid, 1 if invalid
async fn validate_config_only() {
    match try_validate_config().await {
        Ok(_) => {
            println!("Configuration is valid");
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Configuration validation failed: {}", e);
            process::exit(1);
        }
    }
}

/// Try to validate the configuration, returning an error if invalid
/// This function is testable (does not call process::exit)
async fn try_validate_config() -> Result<(), config::loader::ConfigError> {
    config::load_config().await.map(|_| ())
}

/// Wait for shutdown signals (SIGINT or SIGTERM)
async fn wait_for_shutdown() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::signal::unix::{signal, SignalKind};

    // Create signal handlers
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down gracefully...");
        }
        _ = sigterm.recv() => {
            info!("Received SIGTERM, shutting down gracefully...");
        }
        _ = sigint.recv() => {
            info!("Received SIGINT, shutting down gracefully...");
        }
    }

    Ok(())
}

fn init_logging(config: &LoggingConfig) {
    use tracing_subscriber::EnvFilter;

    // Priority: RUST_LOG env var > config.level > default "info"
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(config.level.as_str()));

    // Initialize with the configured format
    match config.format {
        LogFormat::Json => {
            tracing_subscriber::fmt()
                .json()
                .with_env_filter(filter)
                .init();
        }
        LogFormat::Pretty => {
            tracing_subscriber::fmt()
                .pretty()
                .with_env_filter(filter)
                .init();
        }
        LogFormat::Compact => {
            tracing_subscriber::fmt()
                .compact()
                .with_env_filter(filter)
                .init();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_validate_config_success() {
        let yaml = r#"
proxy:
  port: 9090
  host: "127.0.0.1"
  max_connections: 5000
  timeout_seconds: 60
ca:
  cert_path: "/etc/ca.crt"
  key_path: "/etc/ca.key"
  cache_size: 500
  cache_ttl_hours: 12
audit:
  output: "file"
  file_path: "/var/log/audit.json"
  redact_pii: true
  level: "debug"
logging:
  level: "debug"
  format: "pretty"
"#;

        // Create a temp file in the project directory for path validation
        let temp_file = NamedTempFile::new_in(".").unwrap();
        let path = temp_file.path().to_path_buf();
        fs::write(&path, yaml).unwrap();

        // Set the config path environment variable
        env::set_var("CONFIG_PATH", &path);

        // Should succeed with valid config
        let result = try_validate_config().await;
        assert!(
            result.is_ok(),
            "Expected config validation to succeed, got: {:?}",
            result
        );

        // Clean up
        env::remove_var("CONFIG_PATH");
    }

    #[tokio::test]
    async fn test_validate_config_failure_invalid_yaml() {
        // Create invalid YAML with unclosed quote
        let yaml = "{\n  \"unclosed: value\n}";

        // Create a temp file in the project directory for path validation
        let temp_file = NamedTempFile::new_in(".").unwrap();
        let path = temp_file.path().to_path_buf();
        fs::write(&path, yaml).unwrap();

        // Set the config path environment variable
        env::set_var("CONFIG_PATH", &path);

        // Should fail with invalid YAML
        let result = try_validate_config().await;
        assert!(result.is_err(), "Expected config validation to fail");

        // Clean up
        env::remove_var("CONFIG_PATH");
    }

    #[tokio::test]
    async fn test_validate_config_failure_missing_file() {
        // Set a non-existent config path within the project directory
        env::set_var("CONFIG_PATH", "./nonexistent_config_file.yaml");

        // Should fail with file not found
        let result = try_validate_config().await;
        assert!(result.is_err(), "Expected config validation to fail");

        // Verify it's a NotFound error
        match result.unwrap_err() {
            config::loader::ConfigError::NotFound(_) => (), // expected
            _ => panic!("Expected NotFound error"),
        }

        // Clean up
        env::remove_var("CONFIG_PATH");
    }
}
