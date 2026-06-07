#![allow(missing_docs)]

use cirious_codex_cli::{CodexCommand, GlobalArgs};
use clap::{Parser, Subcommand};

// 1. Mock CLI for testing
#[derive(Parser, Debug)]
#[command(name = "test_app")]
struct TestCLI {
  #[command(flatten)]
  global: GlobalArgs,

  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
  Start { port: u16 },
  Ping,
}

impl CodexCommand for TestCLI {
  fn global_args(&self) -> &GlobalArgs {
    &self.global
  }
}

// 2. The Tests

#[test]
fn test_global_flags_extraction() {
  // Simulate user typing in the terminal
  let mock_args = vec!["test_app", "--verbose", "--config", "env.toml", "start", "8080"];

  // try_parse_from prevents the app from crashing on error during tests
  let parsed_cli = TestCLI::try_parse_from(mock_args).expect("Failed to parse valid CLI arguments");

  // Validate that our engine's GlobalArgs were intercepted correctly
  let globals = parsed_cli.global_args();
  assert!(globals.verbose, "The verbose flag should be true");
  assert_eq!(
    globals.config.as_deref(),
    Some("env.toml"),
    "The config path did not match"
  );

  // Validate that the specific user command was routed correctly
  match parsed_cli.command {
    Commands::Start { port } => assert_eq!(port, 8080, "The port should be parsed as 8080"),
    _ => panic!("The router selected the wrong sub-command!"),
  }
}

#[test]
fn test_missing_required_subcommand_fails() {
  // Simulate user forgetting the subcommand
  let mock_args = vec!["test_app", "--verbose"];

  let result = TestCLI::try_parse_from(mock_args);

  // We expect clap to throw an error, preventing execution
  assert!(
    result.is_err(),
    "Clap should have rejected the input due to a missing subcommand"
  );
}
