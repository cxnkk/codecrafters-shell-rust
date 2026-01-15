pub fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escape_next = false;

    for c in input.chars() {
        if in_single_quotes {
            if c == '\'' {
                in_single_quotes = false;
            } else {
                current_arg.push(c);
            }
        } else if escape_next {
            escape_next = false;

            if in_double_quotes {
                match c {
                    '$' | '`' | '"' | '\\' | '\n' => {
                        current_arg.push(c);
                    }
                    _ => {
                        current_arg.push('\\');
                        current_arg.push(c);
                    }
                }
            } else {
                current_arg.push(c);
            }
        } else {
            match c {
                '\\' => escape_next = true,
                '\'' => {
                    if in_double_quotes {
                        current_arg.push(c);
                    } else {
                        in_single_quotes = true;
                    }
                }
                '"' => in_double_quotes = !in_double_quotes,
                ' ' | '\t' | '\n' | '\r' => {
                    if in_double_quotes {
                        current_arg.push(c);
                    } else if !current_arg.is_empty() {
                        args.push(current_arg);
                        current_arg = String::new();
                    }
                }
                _ => current_arg.push(c),
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}
