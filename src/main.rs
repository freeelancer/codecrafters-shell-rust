#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::PathBuf, process};

pub enum Command<'a> {
    Exit(i32),
    Echo(Vec<&'a str>),
    Type(&'a str),
    Cd(&'a str),
    Pwd(),
    NotBuiltin(&'a str, Vec<&'a str>),
}

pub fn into_command(raw_args: &str) -> Command {
    let parsed_args: Vec<&str> = raw_args.split_whitespace().collect::<Vec<&str>>();
    let cmd = parsed_args[0];
    match cmd {
        "exit" => Command::Exit(parsed_args[1].parse::<i32>().unwrap()),
        "echo" => Command::Echo(parsed_args),
        "type" => Command::Type(parsed_args[1]),
        "pwd" => Command::Pwd(),
        "cd" => Command::Cd(parsed_args.get(1).unwrap_or(&"")),
        _ => Command::NotBuiltin(cmd, parsed_args[1..].to_vec()),
    }
}

impl<'a> Command<'a> {
    pub fn execute(self) {
        match self {
            Command::Exit(code) => std::process::exit(code),
            Command::Echo(args) => {
                let echo_string = args[1..].join(" ");
                println!("{}", echo_string)
            }
            Command::Type(command) => match command {
                "exit" | "echo" | "type" => println!("{} is a shell builtin", command),
                _ => println!("{}", check_bin(command)),
            },
            Command::Cd(path) => {
                if path == "~" {
                    let home = env::var("HOME").unwrap();
                    if let Err(err) = env::set_current_dir(home) {
                        println!("{}", err.to_string());
                    }
                    return;
                }
                if let Err(_) = env::set_current_dir(path) {
                    println!("cd: {}: No such file or directory", path);
                }
            }
            Command::Pwd() => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            }
            Command::NotBuiltin(command, args) => {
                if let Err(err) = run_bin(command, args) {
                    println!("{}", err);
                }
            }
        }
    }
}

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

        let command = into_command(&input);
        command.execute();
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
        process::Command::new(path)
            .args(args)
            .status()
            .expect("failed to execute process");
        return Ok(());
    }
    return Err(format!("{}: not found", file_name));
}
