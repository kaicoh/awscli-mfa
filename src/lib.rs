use anyhow::Error;

pub mod aws;
pub mod cmd;
mod config;

pub type Result<T> = std::result::Result<T, Error>;
pub use config::Config;

pub fn get_otp(config: &Config, profile: &str) -> Result<String> {
    let secret = config.get_secret(profile)?;
    otp::make_totp(&secret.to_ascii_uppercase(), 30, 0)
        .map(|pass| format!("{pass}"))
        .map_err(anyhow::Error::new)
}
