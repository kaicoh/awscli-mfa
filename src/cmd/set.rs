use crate::{MfaConfig, Result};

#[derive(clap::Args)]
pub struct Args {
    /// Profile name
    #[arg(short, long)]
    profile: String,

    /// Secret for the MFA device
    #[arg(short, long)]
    secret: String,
}

pub fn run(config: MfaConfig, args: &Args) -> Result<()> {
    let Args { profile, secret } = args;
    config.set(profile, secret).save()?;
    println!("Saved the secret key for profile \"{profile}\" successfully.");
    Ok(())
}
