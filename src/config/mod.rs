//! Configuration module for loading and validating YAML configuration files

pub mod loader;
pub mod types;

pub use loader::load_config;
pub use types::Config;
