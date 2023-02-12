use anyhow::{anyhow, Error};
use totp_rs::{Algorithm, Secret, TOTP};

pub mod aws;
pub mod cmd;
mod config;

pub type Result<T> = std::result::Result<T, Error>;
pub use config::Config;

pub fn get_otp(config: &Config, profile: &str) -> Result<String> {
    let secret = Secret::Encoded(config.get_secret(profile)?.to_ascii_uppercase())
        .to_bytes()
        .map_err(|e| anyhow!("{:#?}", e))?;
    TOTP::new(Algorithm::SHA1, 6, 1, 30, secret)
        .map_err(|e| anyhow!("{:#?}", e))
        .and_then(|totp| totp.generate_current().map_err(Error::new))
}
