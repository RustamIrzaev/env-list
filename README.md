# Env List

## Overview

`env-list` is a command-line utility that parses and displays environment variables on `macOS` and `Windows`:
- all environment variables, 
- a filtered list of common environment variables, 
- or only the PATH variable.

## Installation

To use this utility, you'll need to have Rust installed on your system. If you don't have Rust installed, you can get it from [https://rust-lang.org/](https://rust-lang.org/).

1. Clone the repository:

    ```bash
    git clone https://github.com/RustamIrzaev/env-list.git
    cd env-list
    ```

2. Build the project:

    ```bash
    cargo build --release
    cd target/release
    ```

3. Run the project:

    ```bash
    ./env-list <PARAMS>
    ```

## Usage

The utility provides several options to customize the output.

### Display All Environment Variables

By default, `env-list` will display all environment variables:

`./env-list`

### Common Environment Variables

Use the `-simple` (or `-s`) flag to display only common environment variables:

`./env-list -s`

> Common environment variables are: 
> 
> `MAC`: "PATH", "HOME", "SHELL", "USER", "LOGNAME", "LANG", "PWD", "OLDPWD", "SHLVL", "_", "TERM", "TMPDIR"
> 
> `WINDOWS`: "APPDATA", "COMPUTERNAME", "HOMEDRIVE", "HOMEPATH", "OS", "Path", "ProgramData", "ProgramFiles", "ProgramFiles(x86)", "SESSIONNAME", "SystemDrive", "USERDOMAIN", "USERPROFILE"

### Display Only the PATH Variable

Use the `-path` (or `-p`) flag to display only the PATH environment variable:

`./env-list -p`

## Example output (`-s` option)

```sh
PATH:
    /usr/local/bin
    /usr/bin
    /bin
    /usr/sbin
    /sbin
HOME: /Users/yourusername
SHELL: /bin/zsh
USER: yourusername
LOGNAME: yourusername
LANG: en_US.UTF-8
PWD: /Users/yourusername
OLDPWD: /Users
SHLVL: 1
_: /usr/bin/env
TERM: xterm-256color
TMPDIR: /var/folders/...
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
