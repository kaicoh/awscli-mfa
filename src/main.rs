use awscli_mfa::aws::{Config as AwsConfig, Credential, GetSessionToken};
use awscli_mfa::{cmd, get_otp, Config, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "awsmfa")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Profile name executing mfa action for.
    #[arg(short, long)]
    profile: Option<String>,

    /// Duration seconds that the credentials should remain valid..
    #[arg(short, long)]
    duration: Option<i32>,

    /// Commands to read or write config file.
    #[command(subcommand)]
    command: Option<cmd::Commands>,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::new()?;

    match &cli.command {
        Some(cmd::Commands::Ls) => cmd::ls::run(config),
        Some(cmd::Commands::Set(args)) => cmd::set::run(config, args),
        Some(cmd::Commands::Otp(args)) => cmd::otp::run(config, args),
        None => {
            let opt_profile = cli.profile;
            let opt_duration = cli.duration;
            let profile = opt_profile.clone().unwrap_or("default".to_string());

            let serial_number = config.get_arn(&profile)?;
            let token_code = get_otp(&config, &profile)?;

            let sts_cred = GetSessionToken::new()
                .set_profile(opt_profile)
                .set_duration_seconds(opt_duration)
                .set_serial_number(Some(serial_number))
                .set_token_code(Some(token_code))
                .send()
                .await?;

            let credential = Credential::from_sts_cred("mfa", sts_cred);

            AwsConfig::new()?
                .set(credential)
                .save()
        }
    }
}
