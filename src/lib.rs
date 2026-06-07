//! # Cirious Codex CLI
//!
//! The premier entrypoint library for building powerful, production-ready Command
//! Line Interfaces (CLIs) and microservices within the Cirious ecosystem.
//!
//! This crate automates the entire bootstrapping process. Instead of writing
//! boilerplate code to parse arguments, instantiate loggers, and load environment
//! configurations, `cirious_codex_cli` handles it seamlessly. It orchestrates
//! `cirious_codex_config` and `cirious_codex_logger` out of the box, allowing
//! developers to focus purely on business logic.
//!
//! ## Quick Start
//!
//! Define your CLI structure using `clap` and embed `GlobalArgs`. Then,
//! implement the `CodexCommand` trait and use `execute_cli` to bootstrap
//! the ecosystem and route your commands.
//!
//! ```no_run
//! use cirious_codex_cli::{execute_cli, CodexCommand, GlobalArgs};
//! use clap::{Parser, Subcommand};
//!
//! #[derive(Parser, Debug)]
//! #[command(name = "my_microservice")]
//! struct AppCLI {
//!     #[command(flatten)]
//!     global: GlobalArgs,
//!
//!     #[command(subcommand)]
//!     command: Commands,
//! }
//!
//! #[derive(Subcommand, Debug)]
//! enum Commands {
//!     Start { port: u16 },
//!     Ping,
//! }
//!
//! impl CodexCommand for AppCLI {
//!     fn global_args(&self) -> &GlobalArgs {
//!         &self.global
//!     }
//! }
//!
//! fn main() {
//!     execute_cli::<AppCLI, _>(|parsed_cli| match parsed_cli.command {
//!         Commands::Start { port } => println!("Starting server on port {}", port),
//!         Commands::Ping => println!("Pong!"),
//!     });
//! }
//! ```

#![warn(missing_docs)]

/// Core argument parsing and command routing definitions.
///
/// This module provides the essential structures and traits needed to intercept
/// global flags and bootstrap the application.
pub mod args;

#[cfg(feature = "config")]
pub use args::execute_cli_with_config;

pub use args::{execute_cli, CodexCommand, GlobalArgs};
