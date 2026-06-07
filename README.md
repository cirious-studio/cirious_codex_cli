<div align="center">

# 🚀 Cirious Codex CLI

**Rapid Command Line Application Scaffolding for the Cirious Ecosystem**

[![CI](https://github.com/cirious-studio/cirious_codex_cli/actions/workflows/ci.yml/badge.svg)](https://github.com/cirious-studio/cirious_codex_cli/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/cirious_codex_cli.svg)](https://crates.io/crates/cirious_codex_cli) [![Docs.rs](https://docs.rs/cirious_codex_cli/badge.svg)](https://docs.rs/cirious_codex_cli) [![Language](https://img.shields.io/badge/Language-Rust-black?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT%2FApache-blue.svg)](#-license)

</div>

---

## 📖 Overview

**Cirious Codex CLI** is the premier entrypoint library for building powerful, production-ready Command Line Interfaces (CLIs) and microservices within the Cirious ecosystem.

Instead of writing boilerplate code to parse arguments, instantiate loggers, and load environment configurations, `cirious_codex_cli` automates the entire bootstrapping process. It seamlessly connects `cirious_codex_config` and `cirious_codex_logger` out of the box, allowing you to focus purely on your business logic.

---

## ✨ Features

- **Argument Parsing & Sub-command Routing**: Elegant, type-safe CLI definition via `clap` derive macros with automatic `--help` generation and strict validation.
- **Automated Logger Bootstrapping**: The `--verbose` flag automatically switches the `cirious_codex_logger` between `Info` and `Debug` levels, with styled terminal output via `cirious_codex_term`.
- **Automated Configuration Loading**: The `--config <path>` flag triggers `cirious_codex_config` to read, parse, and merge the configuration file with environment variables, delivering a strongly-typed struct to your handler.
- **CLI Override Pattern**: Arguments passed directly in the terminal can seamlessly override values from the configuration file, following the standard 12-factor app methodology.
- **Ecosystem Integration**: Built to leverage the full power of the `cirious_codex` facade, including `CodexError` and `CodexOk`.

---

## 🚀 Quick Start

Add the following to your `Cargo.toml`:

```toml
[dependencies]
cirious_codex_cli = "0.1.0"
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
```

### Example 1: Basic Routing

Define your CLI, implement the `CodexCommand` contract, and let the engine bootstrap everything.

```rust
use cirious_codex_cli::{execute_cli_with_config, CodexCommand, GlobalArgs};
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyConfig {
    database_url: String,
}

#[derive(Parser, Debug)]
#[command(name = "my_app")]
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
            println!("Connecting to database: {}", cfg.database_url);
        }
        match parsed_cli.command {
            Commands::Start { port } => println!("Starting server on port {}", port),
            Commands::Ping => println!("Pong!"),
        }
    });
}
```

Run it:
```bash
# Standard mode (Info level logging)
cargo run --example 01_basic_usage -- --config examples/resources/config.json start 8080

# Verbose mode (Debug level logging)
cargo run --example 01_basic_usage -- --verbose --config examples/resources/config.json start 8080
```

---

### Example 2: Production Microservice (CLI Override Pattern)

```rust
use cirious_codex_cli::{execute_cli_with_config, CodexCommand, GlobalArgs};
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MicroserviceConfig {
    pub server_port: u16,
    pub database_url: String,
    pub worker_threads: usize,
}

#[derive(Parser, Debug)]
#[command(name = "auth_service", version = "1.0", about = "Cirious Authentication Microservice")]
struct AuthCLI {
    #[command(flatten)]
    global: GlobalArgs,

    #[command(subcommand)]
    command: AuthCommands,
}

#[derive(Subcommand, Debug)]
enum AuthCommands {
    /// Starts the HTTP server
    Serve {
        /// Overrides the server port from the configuration file
        #[arg(short, long)]
        port: Option<u16>,
    },
    /// Runs database migrations
    Migrate {
        /// Drop all tables before migrating (DANGEROUS)
        #[arg(long)]
        reset: bool,
    },
    /// Starts a background worker
    Worker,
}

impl CodexCommand for AuthCLI {
    fn global_args(&self) -> &GlobalArgs {
        &self.global
    }
}

fn main() {
    execute_cli_with_config::<AuthCLI, MicroserviceConfig, _>(|cli, config_opt| {
        let config = config_opt.expect("CRITICAL: A valid configuration file is required!");

        match cli.command {
            AuthCommands::Serve { port } => {
                // CLI argument overrides configuration file value
                let active_port = port.unwrap_or(config.server_port);
                println!("Starting HTTP Server on port {} ({} threads)", active_port, config.worker_threads);
            }
            AuthCommands::Migrate { reset } => {
                if reset { println!("WARNING: Dropping all tables!"); }
                println!("Executing SQL migrations...");
            }
            AuthCommands::Worker => println!("Starting background job processor..."),
        }
    });
}
```

With a `microservice.json` configuration file:
```json
{
    "server_port": 3000,
    "database_url": "postgres://admin:pass@localhost:5432/cirious_auth",
    "worker_threads": 16
}
```

Run it:
```bash
# Serve using port from config (3000)
cargo run --example 02_robust_microservice -- --config examples/resources/microservice.json serve

# Serve overriding port via CLI (8080 takes precedence)
cargo run --example 02_robust_microservice -- --config examples/resources/microservice.json serve --port 8080

# Run destructive migrations with verbose logging
cargo run --example 02_robust_microservice -- --verbose --config examples/resources/microservice.json migrate --reset
```

---

## 🚧 Current Status & Roadmap

### ✅ v0.1.0 — Completed

- [x] Design the argument parsing and sub-command routing API.
- [x] Implement automatic initialization of `cirious_codex_logger` dispatchers based on CLI flags.
- [x] Implement automated configuration loading bridging with `cirious_codex_config`.
- [x] Create robust examples demonstrating rapid application scaffolding.

### 🔭 v0.2.0 — Planned

- [ ] Add a global `init_cli!` macro for zero-boilerplate entrypoints.
- [ ] Implement a `Result`-aware handler variant (`execute_cli_result`) that integrates with `CodexError`.
- [ ] Add support for `APP_*` environment variable overrides via `cirious_codex_config`'s env prefix API.
- [ ] Publish `cirious_codex_cli` to `crates.io` as a standalone crate.

---

## 📜 License

Licensed under either of the following, at your option:

* **[MIT License](LICENSE-MIT)**
* **[Apache License 2.0](LICENSE-APACHE)**

---

<div align="center">
  <i>Minimalist by design. Consistent in execution.</i><br>
  <sub>Engineered by Cirious Studio</sub>
</div>
