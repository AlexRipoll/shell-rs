use std::env::{self, current_dir, set_current_dir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

use home::home_dir;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let path = env::var("PATH").expect("PATH env not defined");

        let args: Vec<&str> = input.split_whitespace().collect();
        let cmd = args[0];

        match cmd.to_shell_cmd() {
            ShellCmd::Builtin(kind) => match kind {
                BuiltinCmd::Echo => {
                    echo(args[1..].to_vec());
                }
                BuiltinCmd::Type => {
                    type_of(args[1], path.as_str());
                }
                BuiltinCmd::Pwd => {
                    pwd();
                }
                BuiltinCmd::Cd => cd(args[1]),
                BuiltinCmd::Exit => {
                    if let Some(status_code) = args.get(1) {
                        let status_code = status_code.parse::<i32>().expect("invalid status code");
                        exit(status_code);
                    }
                    exit(0);
                }
            },
            ShellCmd::Binary => {
                exec_binary(cmd, args, path.as_str());
            }
        }
    }
}

#[derive(Debug)]
enum ShellCmd {
    Builtin(BuiltinCmd),
    Binary,
}

#[derive(Debug)]
enum BuiltinCmd {
    Echo,
    Type,
    Exit,
    Pwd,
    Cd,
}

trait ShellCmdExt {
    /// Converts a command string to a `ShellCmd` variant.
    fn to_shell_cmd(&self) -> ShellCmd;
}

impl ShellCmdExt for &str {
    fn to_shell_cmd(&self) -> ShellCmd {
        match *self {
            "echo" => ShellCmd::Builtin(BuiltinCmd::Echo),
            "type" => ShellCmd::Builtin(BuiltinCmd::Type),
            "exit" => ShellCmd::Builtin(BuiltinCmd::Exit),
            "pwd" => ShellCmd::Builtin(BuiltinCmd::Pwd),
            "cd" => ShellCmd::Builtin(BuiltinCmd::Cd),
            _ => ShellCmd::Binary,
        }
    }
}

/// Executes the `echo` built-in command, which prints its arguments to the standard output.
///
/// # Arguments
///
/// * `args` - A vector of arguments to be printed.
fn echo(args: Vec<&str>) {
    println!("{}", args.join(" "));
}

/// Executes the `pwd` built-in command, which prints the current working directory.
/// It handles errors that may occur while retrieving the current directory.
fn pwd() {
    match current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("{}", e),
    }
}

/// Executes the `cd` built-in command, which changes the current working directory.
/// If the provided directory is empty, it defaults to the current directory.
/// It supports tilde (`~`) expansion for the home directory and handles errors that may occur.
///
/// # Arguments
///
/// * `dir` - The directory to change to. If empty, defaults to the current directory.
fn cd(dir: &str) {
    // gets path from arguments, if it's not provided defaults to the current directory
    let mut path = if dir.is_empty() {
        match current_dir() {
            Ok(current_dir) => current_dir,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    } else {
        PathBuf::from(dir)
    };

    if path.starts_with("~") {
        path = match home_dir() {
            Some(home_dir) => match path.strip_prefix("~") {
                Ok(stripped) => home_dir.join(stripped),
                Err(_) => {
                    eprintln!("failed to strip prefix from path");
                    return;
                }
            },
            None => {
                eprintln!("HOME directory not found");
                return;
            }
        }
    }

    if set_current_dir(path.clone()).is_err() {
        eprintln!("cd: {}: No such file or directory", path.display());
    }
}

/// Executes the `type` built-in command, which prints whether the command is a built-in or a binary.
///
/// # Arguments
///
/// * `cmd` - The command to check.
/// * `path` - The PATH environment variable to search for the command.
fn type_of(cmd: &str, path: &str) {
    match cmd.to_shell_cmd() {
        ShellCmd::Builtin(_) => println!("{} is a shell builtin", cmd),
        ShellCmd::Binary => {
            if let Some(dir) = search_in_path(cmd, path) {
                let path = format!("{}/{}", dir, cmd);
                println!("{} is {}", cmd, path);
            } else {
                eprintln!("{}: not found", cmd.trim_end());
            }
        }
    };
}

/// Exits the shell with the specified exit code.
///
/// # Arguments
///
/// * `code` - The exit status code.
fn exit(code: i32) {
    process::exit(code);
}

/// Executes an external binary command found in the PATH directories.
///
/// # Arguments
///
/// * `cmd` - The command to execute.
/// * `args` - A vector of arguments to pass to the command.
/// * `path` - The PATH environment variable to search for the command.
fn exec_binary(cmd: &str, args: Vec<&str>, path: &str) {
    // check if the command is a binary stored in one of the PATH directories
    if let Some(dir) = search_in_path(cmd, path) {
        // set executable args
        let mut cmd_args = "";
        if args.len() > 1 {
            cmd_args = args[1];
        }

        // run the executable
        process::Command::new(format!("{}/{}", dir, cmd))
            .arg(cmd_args)
            .status()
            .expect("failed to execute process");
    } else {
        // binary not found in any PATH dir
        eprintln!("{}: command not found", cmd.trim_end());
    }
}

/// Searches for a binary executable in the directories specified by the PATH environment variable.
///
/// # Arguments
///
/// * `binary` - The name of the binary to search for.
/// * `path` - The PATH environment variable to search in.
///
/// # Returns
///
/// Returns an `Option<&str>` containing the directory where the binary was found, or `None` if not found.
fn search_in_path<'a>(binary: &str, path: &'a str) -> Option<&'a str> {
    path.split(':').find(|dir| {
        let binary_path = format!("{}/{}", dir, binary);
        Path::new(&binary_path).try_exists().unwrap_or(false)
    })
}
