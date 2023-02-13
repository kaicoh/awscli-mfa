use crate::{get_otp, MfaConfig, Result};

use anyhow::anyhow;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

#[derive(clap::Args)]
pub struct Args {
    /// Profile name
    #[arg(short, long)]
    profile: Option<String>,

    /// Whether put the one time password to clipboard or not
    #[arg(short, long)]
    clip: bool,
}

pub fn run(config: MfaConfig, args: &Args) -> Result<()> {
    let Args { profile, clip } = args;
    let profile = profile.as_deref().unwrap_or("default");
    let password = get_otp(&config, profile)?;

    println!("{password}");

    if *clip {
        let mut ctx = ClipboardContext::new().map_err(|e| anyhow!("{}", e))?;
        ctx.set_contents(password).map_err(|e| anyhow!("{}", e))?;
    }

    Ok(())
}
