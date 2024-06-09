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
            }
            ["type", arg] => match arg {
                "exit" => println!("exit is a shell builtin"),
                "echo" => println!("echo is a shell builtin"),
                "type" => println!("type is a shell builtin"),
                "ls" => ls(arg),
                _ => println!("{} not found", arg),
            },
            _ => println!("{}: command not found", inputs[0]),
        }
    }
}

fn ls(file_name: &str) {
    // Implement the ls command here
    // get PATH environment variable
    let paths_env: String = std::env::var("PATH").unwrap();
    // split the path into a vector of paths
    let paths: Vec<&str> = paths_env.split(":").collect();
    // iterate over the paths
    for path in paths {
        // check if the file exists in the path
        let file_path = format!("{}/{}", path, file_name);
        if std::fs::metadata(&file_path).is_ok() {
            println!("{} is {}", file_name, file_path.clone());
            return;
        }
    }
    println!("{}: command not found", file_name);
}
