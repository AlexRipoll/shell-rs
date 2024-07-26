use std::env::{self, current_dir, set_current_dir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let path = env::var("PATH").expect("PATH env not defined");

        let args: Vec<&str> = input.trim_end().split_whitespace().collect();
        let cmd = args[0];

        let builtin = match to_builtin(cmd) {
            Ok(x) => x,
            Err(_) => {
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

                continue;
            }
        };

        match builtin {
            Builtin::Echo => {
                let echo = args[1..].join(" ");
                println!("{}", echo);
            }
            Builtin::Type => {
                let binary = args[1];
                match to_builtin(binary) {
                    Ok(_) => println!("{} is a shell builtin", binary),
                    Err(_) => {
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
            Builtin::Pwd => match current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => println!("{}", e),
            },
            Builtin::Cd => {
                // gets path from arguments, if it's not provided defaults to the current directory
                let path = args
                    .get(1)
                    .map(PathBuf::from)
                    .unwrap_or_else(|| current_dir().unwrap());

                if let Err(_) = set_current_dir(path.clone()) {
                    eprintln!("cd: {}: No such file or directory", path.display());
                }
            }
            Builtin::Exit => {
                if let Some(status_code) = args.get(1) {
                    let status_code = status_code.parse::<i32>().expect("invalid status code");
                    exit(status_code);
                }
                exit(1);
            }
        }
    }
}

fn to_builtin(input: &str) -> Result<Builtin, &str> {
    match input {
        "echo" => Ok(Builtin::Echo),
        "type" => Ok(Builtin::Type),
        "exit" => Ok(Builtin::Exit),
        "pwd" => Ok(Builtin::Pwd),
        "cd" => Ok(Builtin::Cd),
        _ => Err("not a builtin"),
    }
}

#[derive(Debug)]
enum Builtin {
    Echo,
    Type,
    Exit,
    Pwd,
    Cd,
}
