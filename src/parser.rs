use crate::{board::Board, command::Command, object::Object};

/// Expected format for `line` is `width,height,x,y`
pub fn parse_header(line: &str) -> (Board, Object) {
    let parts = line.split(",").collect::<Vec<_>>();

    let width: isize = parts[0].parse().expect("Width to be a number");
    let height: isize = parts[1].parse().expect("Height to be a number");
    let board = Board::new(width, height);

    let x: isize = parts[2].parse().expect("x to be a number");
    let y: isize = parts[3].parse().expect("y to be a number");
    let object = Object::new(x, y);

    (board, object)
}

pub fn parse_body(line: &str) -> Vec<Command> {
    line.split(",").map(|cmd| cmd.into()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_header() {
        let (board, object) = parse_header("1,22,333,4444");

        assert_eq!(board.width, 1);
        assert_eq!(board.height, 22);
        assert_eq!(object.x, 333);
        assert_eq!(object.y, 4444);
    }

    #[test]
    #[should_panic]
    fn parse_invalid_header() {
        parse_header("a,b,c,d");
    }

    #[test]
    fn parse_valid_body() {
        let body = parse_body("1,2,3,4,0");

        assert_eq!(body[0], Command::MoveForward);
        assert_eq!(body[1], Command::MoveBackwards);
        assert_eq!(body[2], Command::RotateClockwise);
        assert_eq!(body[3], Command::RotateCounterclockwise);
        assert_eq!(body[4], Command::Quit);
    }

    #[test]
    #[should_panic]
    fn parse_invalid_valid_body() {
        parse_body("1,2,3,4,5");
    }
}
