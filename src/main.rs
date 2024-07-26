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
                let arg = parsed_input[1];
                match builtin(arg) {
                    Ok(_) => println!("{} is a shell builtin", arg),
                    Err(_) => {
                        match env::var("PATH") {
                            Ok(path) => {
                                let dirs: Vec<&Path> =
                                    path.split(':').map(|s| Path::new(s)).collect();

                                let mut found = false;
                                for dir in dirs {
                                    if is_in_dir(arg, dir) {
                                        println!("{} is {:?}", arg, dir);
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    eprintln!("{}: not found", arg.trim_end());
                                }
                            }
                            Err(e) => panic!("{}", e),
                        };
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

fn is_in_dir(file: &str, dir: &Path) -> bool {
    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_name() == file {
                return true;
            }
        }
    }
    false
}
