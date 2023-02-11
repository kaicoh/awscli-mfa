use anyhow::Error;

pub mod cmd;
mod config;

pub type Result<T> = std::result::Result<T, Error>;
pub use config::Config;
