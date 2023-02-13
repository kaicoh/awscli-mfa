use crate::{MfaConfig, Result};

#[derive(clap::Args)]
pub struct Args {
    /// Profile name
    #[arg(short, long)]
    profile: String,
}

pub fn run(config: MfaConfig, args: &Args) -> Result<()> {
    let Args { profile } = args;
    config.remove(profile).save()?;
    println!("Remove the secret key for profile \"{profile}\" successfully.");
    Ok(())
}
