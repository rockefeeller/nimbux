use crate::cli::{Provider, StorageCommand};

pub fn run(cmd: StorageCommand, provider: Option<Provider>) {
    match cmd {
        StorageCommand::List => list(provider),
    }
}

fn list(provider: Option<Provider>) {
    println!("storage list (provider: {provider:?}) — not implemented yet");
}
