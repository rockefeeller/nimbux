use clap::{Parser, Subcommand, ValueEnum};

/// A unified CLI wrapper for AWS, GCP and Azure.
#[derive(Debug, Parser)]
#[command(name = "nimbux", version, about, propagate_version = true)]
pub struct Cli {
    /// Cloud provider to target
    #[arg(short, long, global = true, value_enum)]
    pub provider: Option<Provider>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Provider {
    Aws,
    Gcp,
    Azure,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage compute instances
    #[command(subcommand)]
    Compute(ComputeCommand),

    /// Manage storage buckets
    #[command(subcommand)]
    Storage(StorageCommand),
}

#[derive(Debug, Subcommand)]
pub enum ComputeCommand {
    /// List compute instances
    List,
}

#[derive(Debug, Subcommand)]
pub enum StorageCommand {
    /// List storage buckets
    List,
}
