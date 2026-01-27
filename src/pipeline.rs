use std::process::{Child, Command, Stdio};

use crate::quoting::parse_args;

pub fn run_pipeline(input: &str) {
    let commands: Vec<&str> = input.split('|').collect();

    let mut previous_command_output: Option<Child> = None;

    for (i, raw_command) in commands.iter().enumerate() {
        let parts = parse_args(raw_command.trim());

        if parts.is_empty() {
            continue;
        }

        let stdin = if let Some(mut child) = previous_command_output.take() {
            Stdio::from(child.stdout.take().expect("Failed to open stdout"))
        } else {
            Stdio::inherit()
        };

        let stdout = if i == commands.len() - 1 {
            Stdio::inherit()
        } else {
            Stdio::piped()
        };

        let child = Command::new(&parts[0])
            .args(&parts[1..])
            .stdin(stdin)
            .stdout(stdout)
            .spawn();

        match child {
            Ok(child) => {
                previous_command_output = Some(child);
            }
            Err(_) => {
                println!("{}: command not found", parts[0]);
                return;
            }
        }
    }

    if let Some(mut final_child) = previous_command_output {
        final_child.wait().unwrap();
    }
}
