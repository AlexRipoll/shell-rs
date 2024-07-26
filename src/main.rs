use std::env::{self, current_dir, set_current_dir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::Command;

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
            ShellCmd::Program => {
                // check if the first argument is a binary stored in one of the PATH directoires
                let mut dirs = path.split(':');
                if let Some(dir) = dirs.find(|&dir| {
                    Path::new(format!("{}/{}", dir, cmd).as_str())
                        .try_exists()
                        .is_ok_and(|res| res)
                }) {
                    let cmd_path = format!("{}/{}", dir, cmd);

                    let mut cmd_args = "";
                    if args.len() > 1 {
                        cmd_args = args[1];
                    }

                    Command::new(cmd_path)
                        .arg(cmd_args)
                        .status()
                        .expect("failed to execute process");
                } else {
                    eprintln!("{}: command not found", cmd.trim_end());
                }
            }
            ShellCmd::Builtin(kind) => match kind {
                BuiltinCmd::Echo => {
                    let echo = args[1..].join(" ");
                    println!("{}", echo);
                }
                BuiltinCmd::Type => {
                    let binary = args[1];
                    match binary.to_shell_cmd() {
                        ShellCmd::Builtin(_) => println!("{} is a shell builtin", binary),
                        ShellCmd::Program => {
                            let mut dirs = path.split(':');
                            if let Some(dir) = dirs.find(|&dir| {
                                Path::new(format!("{}/{}", dir, binary).as_str())
                                    .try_exists()
                                    .is_ok_and(|res| res)
                            }) {
                                let path = format!("{}/{}", dir, binary);
                                println!("{} is {}", binary, path);
                            } else {
                                eprintln!("{}: not found", binary.trim_end());
                            }
                        }
                    };
                }
                BuiltinCmd::Pwd => match current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => println!("{}", e),
                },
                BuiltinCmd::Cd => {
                    // gets path from arguments, if it's not provided defaults to the current directory
                    let mut path = args
                        .get(1)
                        .map(PathBuf::from)
                        .unwrap_or_else(|| current_dir().unwrap());

                    if path.clone().starts_with("~") {
                        path = match home_dir() {
                            Some(home_dir) => match path.strip_prefix("~") {
                                Ok(stripped) => home_dir.join(stripped),
                                Err(_) => {
                                    eprintln!("failed to strip prefix from path");
                                    continue;
                                }
                            },
                            None => {
                                eprintln!("Home directory not found");
                                continue;
                            }
                        }
                    }

                    if set_current_dir(path.clone()).is_err() {
                        eprintln!("cd: {}: No such file or directory", path.display());
                    }
                }
                BuiltinCmd::Exit => {
                    if let Some(status_code) = args.get(1) {
                        let status_code = status_code.parse::<i32>().expect("invalid status code");
                        exit(status_code);
                    }
                    exit(1);
                }
            },
        }
    }
}

trait ShellCmdExt {
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
            _ => ShellCmd::Program,
        }
    }
}

#[derive(Debug)]
enum ShellCmd {
    Builtin(BuiltinCmd),
    Program,
}

#[derive(Debug)]
enum BuiltinCmd {
    Echo,
    Type,
    Exit,
    Pwd,
    Cd,
}
