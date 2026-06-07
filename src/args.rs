#[cfg(feature = "config")]
use cirious_codex_config::{format::ConfigFormat, ConfigBuilder};

use cirious_codex_logger::{Dispatcher, Level, Record, StdoutDispatcher, StyledTerminalFormatter};

use clap::{Args, Parser};
use serde::de::DeserializeOwned;
use std::path::Path;

/// Standard global arguments injected into every Cirious CLI application.
///
/// These arguments are intercepted by the engine to bootstrap global configuration
/// and logging before routing to the application's specific commands.
#[derive(Args, Debug, Clone)]
pub struct GlobalArgs {
  /// Enables verbose logging mode (Trace/Debug levels).
  #[arg(short, long, global = true)]
  pub verbose: bool,

  /// Specifies a custom path for the configuration file.
  #[arg(short, long, global = true)]
  pub config: Option<String>,
}

/// The foundational contract for any CLI application in the Cirious ecosystem.
///
/// Implement this trait on your primary `clap::Parser` struct to allow the engine
/// to extract the global arguments for bootstrapping.
pub trait CodexCommand {
  /// Returns a reference to the parsed global arguments.
  fn global_args(&self) -> &GlobalArgs;
}

/// Bootstraps the environment and executes the CLI application.
///
/// This function parses the command line arguments, initializes the global logger
/// based on the provided flags, and then delegates execution to the provided handler.
pub fn execute_cli<T, F>(handler: F)
where
  T: Parser + CodexCommand,
  F: FnOnce(T),
{
  let parsed_cli = T::parse();
  let globals = parsed_cli.global_args();

  let dispatcher = StdoutDispatcher::new(StyledTerminalFormatter);

  let active_level = if globals.verbose { Level::Debug } else { Level::Info };

  dispatcher.dispatch(&Record {
    level: active_level,
    args: format_args!("Bootstrapping Logger in {:?} mode", active_level),
  });

  handler(parsed_cli);
}

/// Bootstraps the environment, initializes logging, and automatically loads
/// the configuration file specified in the `--config` flag.
pub fn execute_cli_with_config<T, C, F>(handler: F)
where
  T: Parser + CodexCommand,
  C: DeserializeOwned,
  F: FnOnce(T, Option<C>),
{
  let parsed_cli = T::parse();
  let globals = parsed_cli.global_args();

  let dispatcher = StdoutDispatcher::new(StyledTerminalFormatter);
  let active_level = if globals.verbose { Level::Debug } else { Level::Info };

  dispatcher.dispatch(&Record {
    level: active_level,
    args: format_args!("Bootstrapping Logger in {:?} mode", active_level),
  });

  let app_config = if let Some(config_path) = &globals.config {
    let path = Path::new(config_path);

    let content =
      std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read config file at: {}", config_path));

    let format = match path.extension().and_then(|e| e.to_str()) {
      #[cfg(feature = "config_toml")]
      Some("toml") => ConfigFormat::Toml,
      #[cfg(feature = "config_yaml")]
      Some("yaml") | Some("yml") => ConfigFormat::Yaml,
      #[cfg(feature = "config")]
      Some("ron") => ConfigFormat::Ron,
      Some("json") => ConfigFormat::Json,
      _ => panic!("Unsupported config extension or missing feature flag (e.g., config_json)"),
    };

    let cfg = ConfigBuilder::new()
      .add_source(&content, format)
      .expect("Failed to parse the configuration file content")
      .value
      .add_env_prefix("APP_")
      .build::<C>()
      .expect("Failed to build the strongly-typed configuration struct")
      .value;

    dispatcher.dispatch(&Record {
      level: Level::Info,
      args: format_args!("Configuration loaded successfully from: {}", config_path),
    });

    Some(cfg)
  } else {
    None
  };

  handler(parsed_cli, app_config);
}
