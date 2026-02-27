//! Configuration types and schema definitions

use serde::{Deserialize, Serialize};

/// Audit output destination
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AuditOutput {
    /// Output to stdout only
    #[default]
    Stdout,
    /// Output to file only
    File,
    /// Output to both stdout and file
    Both,
}

/// Log level for filtering log messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Trace level - most verbose
    Trace,
    /// Debug level
    Debug,
    /// Info level (default)
    #[default]
    Info,
    /// Warning level
    Warn,
    /// Error level - least verbose
    Error,
}

impl LogLevel {
    /// Convert LogLevel to a lowercase string for use with EnvFilter
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

/// Log output format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Human-readable pretty format
    Pretty,
    /// JSON format (default)
    #[default]
    Json,
    /// Compact format
    Compact,
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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
    #[serde(default)]
    pub output: AuditOutput,
    /// File path for audit logs (when output is "file" or "both")
    pub file_path: Option<String>,
    /// Enable PII redaction (default: true)
    #[serde(default = "default_redact_pii")]
    pub redact_pii: bool,
    /// Log level for audit events (default: "info")
    #[serde(default)]
    pub level: LogLevel,
}

fn default_redact_pii() -> bool {
    true
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            output: AuditOutput::default(),
            file_path: None,
            redact_pii: default_redact_pii(),
            level: LogLevel::default(),
        }
    }
}

/// Logging configuration for application logs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LoggingConfig {
    /// Log level: "trace", "debug", "info", "warn", "error" (default: "info")
    #[serde(default)]
    pub level: LogLevel,
    /// Output format: "pretty", "json", "compact" (default: "json")
    #[serde(default)]
    pub format: LogFormat,
}
