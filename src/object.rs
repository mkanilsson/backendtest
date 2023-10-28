use crate::{board::Board, direction::Direction};

pub struct Object {
    pub x: isize,
    pub y: isize,

    pub direction: Direction,
}

impl Object {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
            direction: Direction::North,
        }
    }

    // Returns `true` if the move is legal, otherwise returns `false`
    pub fn move_forward(&mut self, board: &Board) -> bool {
        match self.direction {
            Direction::North => self.y -= 1,
            Direction::South => self.y += 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
        }

        board.is_legal_position(&self)
    }

    pub fn move_backward(&mut self, board: &Board) -> bool {
        match self.direction {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x -= 1,
            Direction::West => self.x += 1,
        }

        board.is_legal_position(&self)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_header;

    use super::*;

    #[test]
    fn move_forward_when_direction_is_north() {
        let (board, mut object) = parse_header("4,4,1,1");

        assert!(object.move_forward(&board));
        assert_eq!(object.x, 1);
        assert_eq!(object.y, 0);
    }

    #[test]
    fn move_backward_when_direction_is_north() {
        let (board, mut object) = parse_header("4,4,1,1");

        assert!(object.move_backward(&board));
        assert_eq!(object.x, 1);
        assert_eq!(object.y, 2);
    }

    #[test]
    fn move_forward_when_direction_is_east() {
        let (board, mut object) = parse_header("4,4,1,1");

        object.direction = Direction::East;

        assert!(object.move_forward(&board));
        assert_eq!(object.x, 2);
        assert_eq!(object.y, 1);
    }

    #[test]
    fn move_backward_when_direction_is_east() {
        let (board, mut object) = parse_header("4,4,1,1");

        object.direction = Direction::East;

        assert!(object.move_backward(&board));
        assert_eq!(object.x, 0);
        assert_eq!(object.y, 1);
    }

    #[test]
    fn move_forward_when_direction_is_west() {
        let (board, mut object) = parse_header("4,4,1,1");

        object.direction = Direction::West;

        assert!(object.move_forward(&board));
        assert_eq!(object.x, 0);
        assert_eq!(object.y, 1);
    }

    #[test]
    fn move_backward_when_direction_is_west() {
        let (board, mut object) = parse_header("4,4,1,1");

        object.direction = Direction::West;

        assert!(object.move_backward(&board));
        assert_eq!(object.x, 2);
        assert_eq!(object.y, 1);
    }

    #[test]
    fn move_backward_when_direction_is_south() {
        let (board, mut object) = parse_header("4,4,1,1");

        object.direction = Direction::South;

        assert!(object.move_forward(&board));
        assert_eq!(object.x, 1);
        assert_eq!(object.y, 2);
    }

    #[test]
    fn move_forward_when_direction_is_south() {
        let (board, mut object) = parse_header("4,4,1,1");

        object.direction = Direction::South;

        assert!(object.move_backward(&board));
        assert_eq!(object.x, 1);
        assert_eq!(object.y, 0);
    }
}
