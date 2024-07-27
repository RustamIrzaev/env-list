use std::collections::BTreeMap;
use std::process::{Command, Output};
use clap::Parser;

#[derive(Parser)]
#[command(name = "env-parser")]
struct Cli {
    #[arg(short = 's', long = "simple")]
    simple: bool,

    #[arg(short = 'p', long = "path")]
    path: bool,
}

fn main() {
    let cli = Cli::parse();

    if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        match run_env_command() {
            Ok(result) => process_env_vars(cli, result),
            Err(e) => eprintln!("\x1b[31mError: {}\x1b[0m", e)
        }
    } else {
        eprintln!("\x1b[31mThis program can only run on macOS and Windows.\x1b[0m")
    }
}

fn run_env_command() -> Result<BTreeMap<String, Vec<String>>, String> {
    let output : Output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .arg("-Command")
            .arg("Get-ChildItem Env: | ForEach-Object { $_.Name + '=' + $_.Value }")
            .output()
            .map_err(|e| format!("Failed to get environment variables: {}", e))?
    } else {
        Command::new("env")
            .output()
            .map_err(|e| format!("Failed to get environment variables: {}", e))?
    };

    if !output.status.success() {
        return Err(format!("System command failed with status: {}", output.status));
    }

    let output_str = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to parse output: {}", e))?;

    parse_env_variables(&output_str)
}

fn parse_env_variables(source: &str) -> Result<BTreeMap<String, Vec<String>>, String> {
    let mut result = BTreeMap::new();
    let separator_char = if cfg!(target_os = "windows") {";"} else {":"};

    for line in source.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let mut values: Vec<String> = value
                .split(separator_char)
                .filter(|v| !v.is_empty())
                .map(|v| v.to_string())
                .collect();
            values.sort();
            result.insert(key.to_string(), values);
        } else {
            return Err(format!("Failed to parse entry: {}", line));
        }
    }

    Ok(result)
}

fn process_env_vars(cli: Cli, vars: BTreeMap<String, Vec<String>>) {
    if cli.simple && cfg!(target_os = "macos") {
        let keys = [
            "PATH", "HOME", "SHELL", "USER", "LOGNAME", "LANG", "PWD", "OLDPWD",
            "SHLVL", "_", "TERM", "TMPDIR",
        ];

        display_result(&vars, Some(&keys));
    } else if cli.path && cfg!(target_os = "macos") {
        display_result(&vars, Some(&["PATH"]));
    } else if cli.simple && cfg!(target_os = "windows") {
        let keys = [
            "APPDATA", "COMPUTERNAME", "HOMEDRIVE", "HOMEPATH", "OS", "Path",
            "ProgramData", "ProgramFiles", "ProgramFiles(x86)", "SESSIONNAME", "SystemDrive",
            "USERDOMAIN", "USERPROFILE"
        ];

        display_result(&vars, Some(&keys));
    } else if cli.path && cfg!(target_os = "windows") {
        display_result(&vars, Some(&["Path"]));
    } else {
        display_result(&vars, None);
    }
}

fn display_result(data: &BTreeMap<String, Vec<String>>, filter_keys: Option<&[&str]>) {
    for (key, values) in data {
        if filter_keys.map_or(true, |keys| keys.contains(&key.as_str())) {
            if values.len() > 1 {
                println!("\x1b[32m{}\x1b[0m:", key);
                for value in values {
                    println!("\t\x1b[34m{}\x1b[0m", value);
                }
            } else if let Some(value) = values.get(0) {
                println!("\x1b[32m{}\x1b[0m: \x1b[34m{}\x1b[0m", key, value);
            }
        }
    }
}