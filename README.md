Solution for the ["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview), where you build your own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands. The current builtin command implemented are:

- echo
- type
- pwd
- cd
- exit

This repository contains a solution for the ["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview) on Codecrafters. The goal of this project is to build a POSIX-compliant shell that can interpret and execute shell commands, handle built-in commands, and run external programs.

## Features

### Supported Built-in Commands

- **echo**: Outputs the strings passed as arguments.
- **type**: Displays information about the command type (built-in or binary).
- **pwd**: Prints the current working directory.
- **cd**: Changes the current directory (supports tilde `~` expansion for `home` directory in paths).
- **exit**: Exits the shell with an optional status code.

### Installation

Clone the repository:

```bash
git clone https://github.com/AlexRipoll/shell-rs
cd shell-rs
cargo build
```
