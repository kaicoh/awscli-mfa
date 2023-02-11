use clap::Subcommand;

pub mod ls;
pub mod otp;
pub mod set;

#[derive(Subcommand)]
pub enum Commands {
    /// List mfa devices.
    Ls,

    /// Set mfa device to config file.
    Set(set::Args),
}
