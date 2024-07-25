#[allow(unused_imports)]
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

        match parsed_input[0] {
            "echo" => {
                let echo = parsed_input[1..].join(" ");
                println!("{}", echo);
            }
            "exit" => {
                if let Some(status_code) = parsed_input.get(1) {
                    let status_code = status_code.parse::<i32>().expect("invalid status code");
                    process::exit(status_code);
                }
                process::exit(1);
            }
            _ => eprintln!("{}: command not found", parsed_input[0].trim_end()),
        }
    }
}
