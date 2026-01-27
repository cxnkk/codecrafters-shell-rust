use std::{env, fs, os::unix::fs::PermissionsExt};

pub fn find_completions(prefix: &str) -> Vec<String> {
    if prefix.contains(" ") {
        return Vec::new();
    }

    let mut matches = Vec::new();

    let builtins = ["exit", "echo", "type", "pwd", "cd"];
    for builtin in builtins {
        if builtin.starts_with(prefix) {
            matches.push(builtin.to_string());
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
                                matches.push(filename);
                            }
                        }
                    }
                }
            }
        }
    }

    matches.sort();
    matches.dedup();
    matches
}

pub fn find_lcp(matches: &[String]) -> String {
    if matches.is_empty() {
        return String::new();
    }

    let mut prefix = matches[0].clone();

    for c in matches.iter() {
        while !c.starts_with(&prefix) {
            if prefix.is_empty() {
                return String::new();
            }

            prefix.pop();
        }
    }

    prefix
}
