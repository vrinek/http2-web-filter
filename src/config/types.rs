//! Configuration types and schema definitions

use serde::{Deserialize, Serialize};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Proxy server settings
    #[serde(default)]
    pub proxy: ProxyConfig,
    /// Certificate Authority settings
    #[serde(default)]
    pub ca: CaConfig,
    /// Audit logging settings
    #[serde(default)]
    pub audit: AuditConfig,
    /// Logging settings
    #[serde(default)]
    pub logging: LoggingConfig,
}

/// Proxy server configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProxyConfig {
    /// Port to listen on (default: 8080)
    #[serde(default = "default_proxy_port")]
    pub port: u16,
    /// Host address to bind to (default: "0.0.0.0")
    #[serde(default = "default_host")]
    pub host: String,
    /// Maximum concurrent connections (default: 10000)
    #[serde(default = "default_max_connections")]
    pub max_connections: usize,
    /// Connection timeout in seconds (default: 30)
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

fn default_proxy_port() -> u16 {
    8080
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_max_connections() -> usize {
    10000
}

fn default_timeout() -> u64 {
    30
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            port: default_proxy_port(),
            host: default_host(),
            max_connections: default_max_connections(),
            timeout_seconds: default_timeout(),
        }
    }
}

/// Certificate Authority configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaConfig {
    /// Path to CA certificate PEM file
    #[serde(default = "default_ca_cert_path")]
    pub cert_path: String,
    /// Path to CA private key PEM file
    #[serde(default = "default_ca_key_path")]
    pub key_path: String,
    /// Maximum number of cached certificates (default: 1000)
    #[serde(default = "default_cache_size")]
    pub cache_size: u64,
    /// Certificate cache TTL in hours (default: 24)
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_hours: u64,
}

fn default_ca_cert_path() -> String {
    "config/ca.crt".to_string()
}

fn default_ca_key_path() -> String {
    "config/ca.key".to_string()
}

fn default_cache_size() -> u64 {
    1000
}

fn default_cache_ttl() -> u64 {
    24
}

impl Default for CaConfig {
    fn default() -> Self {
        Self {
            cert_path: default_ca_cert_path(),
            key_path: default_ca_key_path(),
            cache_size: default_cache_size(),
            cache_ttl_hours: default_cache_ttl(),
        }
    }
}

/// Audit logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditConfig {
    /// Output format: "stdout", "file", or "both" (default: "stdout")
    #[serde(default = "default_audit_output")]
    pub output: String,
    /// File path for audit logs (when output is "file" or "both")
    pub file_path: Option<String>,
    /// Enable PII redaction (default: true)
    #[serde(default = "default_redact_pii")]
    pub redact_pii: bool,
    /// Log level for audit events (default: "info")
    #[serde(default = "default_audit_level")]
    pub level: String,
}

fn default_audit_output() -> String {
    "stdout".to_string()
}

fn default_redact_pii() -> bool {
    true
}

fn default_audit_level() -> String {
    "info".to_string()
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            output: default_audit_output(),
            file_path: None,
            redact_pii: default_redact_pii(),
            level: default_audit_level(),
        }
    }
}

/// Logging configuration for application logs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoggingConfig {
    /// Log level: "trace", "debug", "info", "warn", "error" (default: "info")
    #[serde(default = "default_log_level")]
    pub level: String,
    /// Output format: "pretty", "json", "compact" (default: "json")
    #[serde(default = "default_log_format")]
    pub format: String,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "json".to_string()
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proxy: ProxyConfig::default(),
            ca: CaConfig::default(),
            audit: AuditConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}
