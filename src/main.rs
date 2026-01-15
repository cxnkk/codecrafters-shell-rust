mod quoting;
mod redirection;

use crate::quoting::parse_args;
use crate::redirection::parse_redirection;

use rustyline::Highlighter;
use rustyline::Hinter;
use rustyline::Validator;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::{Context, Editor, Helper};
use std::env::set_current_dir;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio, exit};
use std::result::Result::Ok;
use std::{env, fs};

enum Cmd {
    Exit,
    Echo,
    Type,
    Run,
    Pwd,
    Cd,
}

impl Cmd {
    fn parse(s: &str) -> Self {
        match s {
            "exit" => Cmd::Exit,
            "echo" => Cmd::Echo,
            "type" => Cmd::Type,
            "pwd" => Cmd::Pwd,
            "cd" => Cmd::Cd,
            _ => Cmd::Run,
        }
    }
}

#[derive(Helper, Hinter, Validator, Highlighter)]
struct ShellHelper;

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        if line.contains(" ") {
            return Ok((0, Vec::new()));
        }

        let mut matches = Vec::new();
        let prefix = &line[..pos];

        let builtins = ["exit", "echo"];
        for builtin in builtins {
            if builtin.starts_with(prefix) {
                matches.push(format!("{} ", builtin));
            }
        }

        if let Ok(path_var) = env::var("PATH") {
            for path in path_var.split(":") {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let filename = entry.file_name().into_string().unwrap_or_default();

                        if filename.starts_with(prefix) {
                            if let Ok(metadata) = entry.metadata() {
                                if metadata.permissions().mode() & 0o111 != 0 {
                                    matches.push(format!("{} ", filename));
                                }
                            }
                        }
                    }
                }
            }
        }

        matches.sort();
        matches.dedup();

        Ok((0, matches))
    }
}

fn main() {
    let mut editor = Editor::new().unwrap();
    editor.set_helper(Some(ShellHelper));

    loop {
        let readline = editor.readline("$ ");

        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                let parts: Vec<String> = parse_args(input);
                if parts.is_empty() {
                    continue;
                }

                match Cmd::parse(&parts[0]) {
                    Cmd::Exit => exit(0),
                    Cmd::Echo => {
                        let mut args = parts[1..].to_vec();

                        let (stdout_opt, _stderr_opt) = parse_redirection(&mut args);

                        let output_text = args.join(" ");

                        match stdout_opt {
                            Some(mut file) => writeln!(file, "{}", output_text).unwrap(),
                            None => {
                                println!("{}", output_text);
                            }
                        }
                    }
                    Cmd::Type => match parts[1].as_str() {
                        "exit" | "echo" | "type" | "pwd" | "cd" => {
                            println!("{} is a shell builtin", parts[1])
                        }
                        _ => {
                            let mut found = false;

                            if let Ok(path_var) = env::var("PATH") {
                                for path in path_var.split(':') {
                                    let full_path = format!("{}/{}", path, parts[1]);

                                    if Path::new(&full_path).exists() {
                                        if let Ok(metadata) = fs::metadata(&full_path) {
                                            if metadata.permissions().mode() & 0o111 != 0 {
                                                println!("{} is {}", parts[1], full_path);
                                                found = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            if !found {
                                println!("{}: not found", parts[1]);
                            }
                        }
                    },
                    Cmd::Run => {
                        let program_name = &parts[0];
                        let mut args = parts[1..].to_vec();

                        let (stdout_opt, stderr_opt) = parse_redirection(&mut args);

                        let stdout_dest = match stdout_opt {
                            Some(f) => Stdio::from(f),
                            None => Stdio::inherit(),
                        };

                        let stderr_dest = match stderr_opt {
                            Some(f) => Stdio::from(f),
                            None => Stdio::inherit(),
                        };

                        match Command::new(program_name)
                            .args(args)
                            .stdout(stdout_dest)
                            .stderr(stderr_dest)
                            .spawn()
                        {
                            Ok(mut child) => {
                                let _ = child.wait();
                            }
                            Err(_) => {
                                println!("{}: command not found", input)
                            }
                        }
                    }
                    Cmd::Pwd => {
                        let path = env::current_dir().expect("Not existing");
                        println!("{}", path.display());
                    }
                    Cmd::Cd => match parts[1].as_str() {
                        "~" => {
                            let home = env::home_dir().expect("No home dir found");
                            set_current_dir(home).expect("Failed changing directory")
                        }
                        _ => {
                            if Path::new(&parts[1]).exists() {
                                set_current_dir(&parts[1]).expect("Failed changing directory")
                            } else {
                                println!("cd: {}: No such file or directory", parts[1])
                            }
                        }
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
