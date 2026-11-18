use crate::cli::{ComputeCommand, Provider};

pub fn run(cmd: ComputeCommand, provider: Option<Provider>) {
    match cmd {
        ComputeCommand::List => list(provider),
    }
}

fn list(provider: Option<Provider>) {
    println!("compute list (provider: {provider:?}) — not implemented yet");
}
