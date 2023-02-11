use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "awsmfa")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Profile name executing mfa action for.
    #[arg(short, long)]
    profile: Option<String>,

    /// Commands to read or write config file.
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List mfa devices.
    Ls,

    /// Set mfa device to config file.
    Set(DeviceArgs),
}

#[derive(Args)]
struct DeviceArgs {
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

fn main() {
    let cli = Cli::parse();

    let profile = cli.profile.unwrap_or("default".to_string());

    match &cli.command {
        Some(Commands::Ls) => {
            println!("ls command selected");
        }
        Some(Commands::Set(DeviceArgs { profile, arn, secret })) => {
            println!("set {} to config file. arn: {}, secret: {}", profile, arn, secret);
        }
        None => {
            println!("exec mfa action for profile: {}", profile);
        }
    }
}
