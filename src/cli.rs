use clap::Parser;

#[derive(Parser)]
#[command(name = "env-parser")]
pub struct Cli {
    #[arg(short = 's', long = "simple")]
    pub simple: bool,

    #[arg(short = 'p', long = "path")]
    pub path: bool,
}