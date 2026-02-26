//! Configuration loader - reads YAML configuration from file or env var

use std::env;
use std::path::Path;

use thiserror::Error;

use super::types::Config;

/// Errors that can occur during configuration loading
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse YAML: {0}")]
    ParseError(#[from] serde_yaml::Error),
    #[error("Config file not found: {0}")]
    NotFound(String),
}

/// Load configuration from file
///
/// Priority:
/// 1. CONFIG_PATH environment variable
/// 2. Default path: ./config/default.yaml
pub fn load_config() -> Result<Config, ConfigError> {
    let config_path =
        env::var("CONFIG_PATH").unwrap_or_else(|_| "./config/default.yaml".to_string());

    load_config_from_path(&config_path)
}

/// Load configuration from a specific path
pub fn load_config_from_path<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ConfigError::NotFound(path.to_string_lossy().to_string()));
    }

    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_config() {
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

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml.as_bytes()).unwrap();

        let config = load_config_from_path(temp_file.path()).unwrap();

        assert_eq!(config.proxy.port, 9090);
        assert_eq!(config.proxy.host, "127.0.0.1");
        assert_eq!(config.proxy.max_connections, 5000);
        assert_eq!(config.proxy.timeout_seconds, 60);
        assert_eq!(config.ca.cert_path, "/etc/ca.crt");
        assert_eq!(config.ca.key_path, "/etc/ca.key");
        assert_eq!(config.ca.cache_size, 500);
        assert_eq!(config.ca.cache_ttl_hours, 12);
        assert_eq!(config.audit.output, "file");
        assert_eq!(
            config.audit.file_path,
            Some("/var/log/audit.json".to_string())
        );
        assert!(config.audit.redact_pii);
        assert_eq!(config.audit.level, "debug");
        assert_eq!(config.logging.level, "debug");
        assert_eq!(config.logging.format, "pretty");
    }

    #[test]
    fn test_load_partial_config() {
        let yaml = r#"
proxy:
  port: 9090
ca:
  cert_path: "/etc/ca.crt"
  key_path: "/etc/ca.key"
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml.as_bytes()).unwrap();

        let config = load_config_from_path(temp_file.path()).unwrap();

        assert_eq!(config.proxy.port, 9090);
        assert_eq!(config.proxy.host, "0.0.0.0"); // default
        assert_eq!(config.ca.cert_path, "/etc/ca.crt");
        assert_eq!(config.audit.redact_pii, true); // default
        assert_eq!(config.logging.format, "json"); // default
    }

    #[test]
    fn test_load_invalid_yaml() {
        let yaml = "invalid: : : yaml";

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml.as_bytes()).unwrap();

        let result = load_config_from_path(temp_file.path());
        assert!(result.is_err());

        match result.unwrap_err() {
            ConfigError::ParseError(_) => (), // expected
            _ => panic!("Expected parse error"),
        }
    }

    #[test]
    fn test_load_file_not_found() {
        let result = load_config_from_path("/nonexistent/path/config.yaml");
        assert!(result.is_err());

        match result.unwrap_err() {
            ConfigError::NotFound(_) => (), // expected
            _ => panic!("Expected not found error"),
        }
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert_eq!(config.proxy.port, 8080);
        assert_eq!(config.proxy.host, "0.0.0.0");
        assert_eq!(config.proxy.max_connections, 10000);
        assert_eq!(config.proxy.timeout_seconds, 30);
        assert_eq!(config.ca.cert_path, "config/ca.crt");
        assert_eq!(config.ca.key_path, "config/ca.key");
        assert_eq!(config.ca.cache_size, 1000);
        assert_eq!(config.ca.cache_ttl_hours, 24);
        assert_eq!(config.audit.output, "stdout");
        assert_eq!(config.audit.redact_pii, true);
        assert_eq!(config.audit.level, "info");
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.logging.format, "json");
    }
}
