use std::io::Write;

use crossterm::{
    ExecutableCommand, cursor,
    terminal::{Clear, ClearType},
};

pub enum Direction {
    Up,
    Down,
}

pub fn move_history(
    direction: Direction,
    history: &Vec<String>,
    input_buffer: &mut String,
    history_index: &mut usize,
    stdout: &mut std::io::Stdout,
) {
    match direction {
        Direction::Up => {
            if *history_index > 0 {
                *history_index -= 1;
            } else {
                return;
            }
        }
        Direction::Down => {
            if *history_index < history.len() {
                *history_index += 1;
            } else {
                return;
            }
        }
    }

    stdout.execute(cursor::MoveToColumn(0)).unwrap();
    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
    print!("$ ");

    input_buffer.clear();

    if *history_index < history.len() {
        input_buffer.push_str(&history[*history_index]);
    }

    print!("{input_buffer}");
    stdout.flush().unwrap();
}
