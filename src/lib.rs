use anyhow::Error;

mod config;
pub mod cmd;

pub type Result<T> = std::result::Result<T, Error>;
pub use config::Config;
