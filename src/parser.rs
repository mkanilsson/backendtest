use crate::{board::Board, object::Object};

/// Expected format for `line` is `width,height,x,y`
pub fn parse_header(line: &str) -> (Board, Object) {
    let parts = line.split(",").collect::<Vec<_>>();

    let width: usize = parts[0].parse().expect("Width to be a number");
    let height: usize = parts[1].parse().expect("Height to be a number");
    let board = Board::new(width, height);

    let x: usize = parts[2].parse().expect("x to be a number");
    let y: usize = parts[3].parse().expect("y to be a number");
    let object = Object::new(x, y);

    (board, object)
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
}
