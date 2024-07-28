mod cli;
mod processing;
mod display;

use clap::Parser;
use cli::Cli;
use crate::processing::{filter_env_vars, run_env_command};

fn main() {
    let cli = Cli::parse();

    if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        match run_env_command() {
            Ok(result) => filter_env_vars(cli, result),
            Err(e) => eprintln!("\x1b[31mError: {}\x1b[0m", e)
        }
    } else {
        eprintln!("\x1b[31mThis program can only run on macOS and Windows.\x1b[0m")
    }
}

