use std::io::{self, BufRead};

use board::Board;
use command::Command;
use object::Object;

mod board;
mod command;
mod direction;
mod object;
mod parser;

fn main() {
    // stdin should be 2 lines.
    // First the header, comprised of `width,height,x,y`
    // then, the body, comprised of `cmd1,cmd2,...,cmdn`
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let (board, object) = parser::parse_header(&iterator.next().unwrap().unwrap());
    let body = parser::parse_body(&iterator.next().unwrap().unwrap());

    match run(board, object, body) {
        Some(object) => println!("[{},{}]", object.x, object.y),
        None => println!("[-1,-1]"),
    }
}

/// Returns `Some(object)` if the path is valid otherwise `None`
fn run(board: Board, mut object: Object, commands: Vec<Command>) -> Option<Object> {
    for cmd in commands {
        match cmd {
            Command::Quit => return Some(object),
            Command::MoveForward => {
                if !object.move_forward(&board) {
                    return None;
                }
            }
            Command::MoveBackwards => {
                if !object.move_backward(&board) {
                    return None;
                }
            }
            Command::RotateClockwise => {
                object.direction.rotate_clockwise();
            }
            Command::RotateCounterclockwise => {
                object.direction.rotate_counter_clockwise();
            }
        }
    }

    Some(object)
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_body, parse_header};

    use super::*;

    #[test]
    fn verify_valid_game() {
        let (board, object) = parse_header("4,4,2,2");
        let commands = parse_body("1,4,1,3,2,3,2,4,1,0");

        let result = run(board, object, commands);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn verify_invalid_game() {
        let (board, object) = parse_header("4,4,2,2");
        let commands = parse_body("1,1,1,1,1"); // Move out of the board

        let result = run(board, object, commands);
        assert!(result.is_none());
    }
}
