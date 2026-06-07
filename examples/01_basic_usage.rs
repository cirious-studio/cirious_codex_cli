#![allow(missing_docs)]

use cirious_codex_cli::{execute_cli_with_config, CodexCommand, GlobalArgs};
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyConfig {
  database_url: String,
}

#[derive(Parser, Debug)]
#[command(name = "my_microservice")]
struct AppCLI {
  #[command(flatten)]
  global: GlobalArgs,

  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  Start { port: u16 },
  Ping,
}

impl CodexCommand for AppCLI {
  fn global_args(&self) -> &GlobalArgs {
    &self.global
  }
}

fn main() {
  execute_cli_with_config::<AppCLI, MyConfig, _>(|parsed_cli, app_config| {
    if let Some(cfg) = app_config {
      println!("Configuration loaded! Connecting to database at: {}", cfg.database_url);
    } else {
      println!("No configuration file provided.");
    }

    match parsed_cli.command {
      Commands::Start { port } => {
        println!("Subcommand: Starting server on port {}", port);
      }
      Commands::Ping => {
        println!("Subcommand: Pong!");
      }
    }
  });
}
