#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let trimmed_cmd = command.trim();
        let input: Vec<&str> = trimmed_cmd.splitn(2, " ").collect();
        match input[0] {
            "exit" => exit(0),
            "echo" => println!("{}", input[1]),
            _ => {
                println!("{}: command not found", trimmed_cmd);
            }
        }
    }
}
