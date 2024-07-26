use core::panic;
use std::fs::read_dir;
use std::io::{self, Write};
use std::path::Path;
use std::{env, process};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let parsed_input: Vec<&str> = input.trim_end().split_whitespace().collect();

        let command = match builtin(parsed_input[0]) {
            Ok(x) => x,
            Err(_) => {
                eprintln!("{}: command not found", parsed_input[0].trim_end());
                continue;
            }
        };

        match command {
            Builtin::Echo => {
                let echo = parsed_input[1..].join(" ");
                println!("{}", echo);
            }
            Builtin::Type => {
                let cmd = parsed_input[1];
                match builtin(cmd) {
                    Ok(_) => println!("{} is a shell builtin", cmd),
                    Err(_) => {
                        let path = env::var("PATH").expect("PATH env not defined");
                        let mut dirs = path.split(':');
                        if let Some(dir) = dirs.find(|&dir| {
                            Path::new(format!("{}/{}", dir, cmd).as_str())
                                .try_exists()
                                .is_ok_and(|res| res)
                        }) {
                            let path = format!("{}/{}", dir, cmd);
                            println!("{} is {}", cmd, path);
                        } else {
                            eprintln!("{}: not found", cmd.trim_end());
                        }
                    }
                };
            }
            Builtin::Exit => {
                if let Some(status_code) = parsed_input.get(1) {
                    let status_code = status_code.parse::<i32>().expect("invalid status code");
                    process::exit(status_code);
                }
                process::exit(1);
            }
        }
    }
}

fn builtin(input: &str) -> Result<Builtin, &str> {
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
