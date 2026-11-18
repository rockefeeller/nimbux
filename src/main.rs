mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Compute(cmd) => commands::compute::run(cmd, cli.provider),
        Command::Storage(cmd) => commands::storage::run(cmd, cli.provider),
    }
}
