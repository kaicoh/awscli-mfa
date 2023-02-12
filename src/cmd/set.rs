use crate::{Config, Result};

#[derive(clap::Args)]
pub struct Args {
    /// Profile name
    #[arg(short, long)]
    profile: String,

    /// Arn of mfa device
    #[arg(short, long)]
    arn: String,

    /// Secret for the device
    #[arg(short, long)]
    secret: String,
}

pub fn run(config: Config, args: &Args) -> Result<()> {
    let Args {
        profile,
        arn,
        secret,
    } = args;
    config.set(profile, arn, secret).save()?;
    println!("Saved MFA device for profile \"{profile}\" successfully.");
    Ok(())
}
