use clap::{Parser, Subcommand};

use crate::cli::pull;
use crate::cli::push;

pub mod cli;

/// Manage and share environments
#[derive(Parser, Debug)]
#[command(author, version, about = "Push and pull environments to a remote")]
pub struct Cli {
    // Manage environments
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
#[command(arg_required_else_help = true)]
pub enum Command {
    /// Pull changes from the remote repo
    Pull(pull::Args),

    /// Push changes to the remote repo
    Push(push::Args),
}

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Command::Push(cmd) => push::execute(cmd).await,
            Command::Pull(cmd) => pull::execute(cmd).await,
        }
    } else {
        std::process::exit(2);
    }
}
