use awscli_mfa::{cmd, Config, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "awsmfa")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Profile name executing mfa action for.
    #[arg(short, long)]
    profile: Option<String>,

    /// Commands to read or write config file.
    #[command(subcommand)]
    command: Option<cmd::Commands>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::new()?;
    let profile = cli.profile.unwrap_or("default".to_string());

    match &cli.command {
        Some(cmd::Commands::Ls) => cmd::ls::run(config),
        Some(cmd::Commands::Set(args)) => cmd::set::run(config, args),
        None => {
            println!("exec mfa action for profile: {profile}");
            Ok(())
        }
    }
}
