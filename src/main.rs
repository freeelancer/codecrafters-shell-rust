#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::PathBuf, process::Command};

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
                _ => println!("{}", check_bin(arg)),
            },
            ["pwd", ..] => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            }
            ["cd", path] => {
                if let Err(err) = env::set_current_dir(path) {
                    println!("{}", err);
                }
            }
            [command, ..] => {
                if let Err(err) = run_bin(command, inputs[1..].to_vec()) {
                    println!("{}", err);
                }
            }
            _ => println!("{}: command not found", inputs[0]),
        }
    }
}

fn find_exe(name: &str) -> Option<PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let exe_path = path.join(name);
            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }
    None
}

fn check_bin(file_name: &str) -> String {
    // get PATH environment variable
    let exe = find_exe(file_name);
    match exe {
        Some(path) => format!("{} is {}", file_name, path.to_str().unwrap()),
        None => format!("{}: not found", file_name),
    }
}

fn run_bin(file_name: &str, args: Vec<&str>) -> Result<(), String> {
    // get PATH environment variable
    if let Some(path) = find_exe(file_name) {
        Command::new(path)
            .args(args)
            .status()
            .expect("failed to execute process");
        return Ok(());
    }
    return Err(format!("{}: not found", file_name));
}
