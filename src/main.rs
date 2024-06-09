#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let inputs: Vec<&str> = input.split_whitespace().collect();
        match inputs[..] {
            ["exit", "0"] => break,
            ["echo", ..] => {
                let echo_string = inputs[1..].join(" ");
                println!("{}", echo_string);
            },
            ["type", command] => {
                match command {
                    "exit" => println!("exit is a shell builtin"),
                    "echo" => println!("echo is a shell builtin"),
                    "type" => println!("type is a shell builtin"),
                    _ => println!("{} not found", command),
                }
            }
            _ => println!("{}: command not found", inputs[0]),
        }
    }
}
