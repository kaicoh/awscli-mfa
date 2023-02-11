use anyhow::Error;

mod config;

pub type Result<T> = std::result::Result<T, Error>;
pub use config::Config;
