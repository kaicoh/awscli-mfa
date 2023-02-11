use crate::{Config, Result};

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

pub fn run(config: Config, args: &Args) -> Result<()> {
    let Args { profile, clip } = args;
    let secret = config.get_secret(profile.as_deref().unwrap_or("default"))?;
    let password = otp::make_totp(&secret.to_ascii_uppercase(), 30, 0)
        .map(|pass| format!("{pass}"))
        .map_err(anyhow::Error::new)?;

    println!("{password}");

    if *clip {
        let mut ctx = ClipboardContext::new().map_err(|e| anyhow!("{}", e))?;
        ctx.set_contents(password).map_err(|e| anyhow!("{}", e))?;
    }

    Ok(())
}
