use std::env;
use std::io::{self, stdout, Write};
use std::path::Path;
use std::process::Command;
use std::process::{exit, Stdio};

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

                    let output = Command::new(cmd_path)
                        .arg(cmd_args)
                        .status()
                        .expect("failed to execute process");

                    println!("{}", output);
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
        _ => Err("not a builtin"),
    }
}

#[derive(Debug)]
enum Builtin {
    Echo,
    Type,
    Exit,
}
