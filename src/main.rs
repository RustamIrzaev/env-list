mod cli;
mod display;
mod processing;

use crate::processing::{filter_env_vars, run_env_command};
use clap::Parser;
use cli::Cli;
use colored::*;

fn main() {
    let cli = Cli::parse();

    if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        match run_env_command() {
            Ok(result) => filter_env_vars(cli, result),
            Err(e) => println!("{}", format!("Error: {}", e).red()),
        }
    } else {
        println!("{}", "This program can only run on macOS and Windows".red())
    }
}
