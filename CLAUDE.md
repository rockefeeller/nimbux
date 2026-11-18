# nimbux

A unified CLI, written in Rust, that wraps the native cloud CLIs (`aws`, `gcloud`, `az`) behind one consistent set of commands, so you don't have to remember each provider's syntax when switching between clouds.

The author is learning Rust with this project: explain non-obvious Rust concepts (ownership, error handling, traits) briefly when introducing them, and prefer clear, idiomatic code over clever code. Talk to the user in Spanish.

## Goals

- Daily use at work when switching between AWS, GCP and Azure.
- Share with colleagues; public on GitHub as a portfolio project.
- Learn Rust.

## Approach

- nimbux **does not use cloud SDKs**. It translates its own commands into calls to the native CLIs via `std::process::Command`, reusing their existing authentication and configuration.
- Argument parsing with `clap` (derive API). Already added to `Cargo.toml`.
- Common command surface with a `--cloud aws|gcp|azure` option.

## First commands (MVP)

| nimbux command | AWS | GCP | Azure |
|---|---|---|---|
| `nimbux compute list` | `aws ec2 describe-instances` | `gcloud compute instances list` | `az vm list` |
| `nimbux storage list` | `aws s3 ls` | `gcloud storage ls` | `az storage account list` |
| `nimbux compute connect <instance>` | `aws ssm start-session --target <id>` | `gcloud compute ssh <name>` (optionally `--tunnel-through-iap`) | `az network bastion ssh` |

Notes:
- AWS and GCP are the priority; the author uses them daily. Azure is wanted but the author has never used it, so explain Azure specifics when implementing them.
- `connect` is interactive: the child process must inherit the terminal (stdin/stdout/stderr) rather than having its output captured.
- Cloud concepts don't map 1:1 (AWS accounts/profiles, GCP projects, Azure subscriptions); keep that in mind when designing shared options.

## Suggested structure

- `src/main.rs` — clap definitions and dispatch.
- `src/providers/` — one module per cloud (`aws.rs`, `gcp.rs`, `azure.rs`), ideally behind a shared trait.
- Clear error when the underlying CLI isn't installed or not in `PATH`.

## Conventions

- Run `cargo fmt` and `cargo clippy` before committing.
- Commit messages in English.

## Planned (not yet done)

- License: MIT (possibly dual MIT/Apache-2.0 later).
- CI with GitHub Actions: fmt, clippy, tests on each PR.
- Release binaries for macOS, Linux and Windows built in CI (cargo-dist).
- Publish to crates.io once there is a first working version.

## Environment

- Developed on macOS (Apple Silicon), Rust installed via rustup.
- Repo: `rockefeeller/nimbux` on GitHub. Git identity for this repo is already configured with the GitHub noreply email.
