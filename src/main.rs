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
                    echo(args);
                }
                BuiltinCmd::Type => {
                    type_of(args[1], path);
                }
                BuiltinCmd::Pwd => {
                    pwd();
                }
                BuiltinCmd::Cd => cd(args.get(1)),
                BuiltinCmd::Exit => {
                    if let Some(status_code) = args.get(1) {
                        let status_code = status_code.parse::<i32>().expect("invalid status code");
                        exit(status_code);
                    }
                    exit(0);
                }
            },
            ShellCmd::Program => {
                exec_program(cmd, args, path);
            }
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

fn echo(args: Vec<&str>) {
    let echo = args[1..].join(" ");
    println!("{}", echo);
}

fn pwd() {
    match current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("{}", e),
    }
}

fn cd(path: Option<&&str>) {
    // gets path from arguments, if it's not provided defaults to the current directory
    let mut path = path
        .map(PathBuf::from)
        .unwrap_or_else(|| current_dir().unwrap());

    if path.clone().starts_with("~") {
        path = match home_dir() {
            Some(home_dir) => match path.strip_prefix("~") {
                Ok(stripped) => home_dir.join(stripped),
                Err(_) => {
                    eprintln!("failed to strip prefix from path");
                    return;
                }
            },
            None => {
                eprintln!("Home directory not found");
                return;
            }
        }
    }

    if set_current_dir(path.clone()).is_err() {
        eprintln!("cd: {}: No such file or directory", path.display());
    }
}

fn type_of(program: &str, path: String) {
    match program.to_shell_cmd() {
        ShellCmd::Builtin(_) => println!("{} is a shell builtin", program),
        ShellCmd::Program => {
            let mut dirs = path.split(':');

            if let Some(dir) = dirs.find(|&dir| {
                Path::new(format!("{}/{}", dir, program).as_str())
                    .try_exists()
                    .is_ok_and(|res| res)
            }) {
                let path = format!("{}/{}", dir, program);
                println!("{} is {}", program, path);
            } else {
                eprintln!("{}: not found", program.trim_end());
            }
        }
    };
}

fn exit(code: i32) {
    process::exit(code);
}

fn exec_program(cmd: &str, args: Vec<&str>, path: String) {
    // check if the command is a binary stored in one of the PATH directories
    let mut dirs = path.split(':');

    if let Some(dir) = dirs.find(|&dir| {
        Path::new(format!("{}/{}", dir, cmd).as_str())
            .try_exists()
            .is_ok_and(|res| res) // check that the binary exists for the given path
    }) {
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
