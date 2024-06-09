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

        // Parse the input
        let (command, arg) = parse_input(&input);
        if command == "exit 0" {
            break;
        } else if command == "echo" {
            println!("{}", arg);
        } else {
            println!("{}: command not found", command);
        }
    }
}

fn parse_input(input: &str) -> (&str, &str) {
    let iter: Vec<&str> = input.split_whitespace().collect();
    if iter.len() == 0 {
        return ("", "");
    } else if iter.len() == 1 {
        return (iter[0].trim(), "");
    }
    return (iter[0].trim(), iter[1].trim());
}
