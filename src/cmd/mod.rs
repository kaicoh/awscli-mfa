use clap::Subcommand;

pub mod ls;
pub mod otp;
pub mod rm;
pub mod set;

#[derive(Subcommand)]
pub enum Commands {
    /// List mfa devices.
    Ls,

    /// Set mfa device to config file.
    Set(set::Args),

    /// Get one time password for provided profile
    Otp(otp::Args),

    /// Remove mfa device from config file.
    Rm(rm::Args),
}
