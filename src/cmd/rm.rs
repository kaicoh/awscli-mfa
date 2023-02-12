use crate::{Config, Result};

#[derive(clap::Args)]
pub struct Args {
    /// Profile name
    #[arg(short, long)]
    profile: String,
}

pub fn run(config: Config, args: &Args) -> Result<()> {
    let Args { profile } = args;
    config.remove(profile).save()?;
    println!("Removed the MFA device for profile \"{profile}\" successfully.");
    Ok(())
}
