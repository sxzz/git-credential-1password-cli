use crate::op::OnePassword;
use clap::{Parser, Subcommand};

mod op;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None,
)]
struct Cli {
    #[arg(help = "The username or 1Password item reference for the username, e.g. username")]
    username: String,
    #[arg(
        help = "The token or 1Password item reference for the token, e.g. op://vault/item/field"
    )]
    token: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Git internal")]
    Get,
    #[command(about = "Git internal")]
    Store,
    #[command(about = "Git internal")]
    Erase,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get => {
            let username = if cli.username.starts_with("op://") {
                OnePassword::get_item(&cli.username)
            } else {
                cli.username
            };
            let token = OnePassword::get_item(&cli.token);

            println!("username={}", username);
            println!("password={}", token);
        }
        Commands::Store => {
            // Do nothing
        }
        Commands::Erase => {
            // Do nothing
        }
    }
}
