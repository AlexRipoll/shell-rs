use std::io::{self, Write};
use std::process;

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
                match builtin(parsed_input[1]) {
                    Ok(_) => println!("{} is a shell builtin", parsed_input[1]),
                    Err(_) => {
                        eprintln!("{}: not found", parsed_input[1].trim_end());
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
