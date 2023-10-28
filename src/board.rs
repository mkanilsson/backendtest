use crate::object::Object;

pub struct Board {
    pub width: isize,
    pub height: isize,
}

impl Board {
    pub fn new(width: isize, height: isize) -> Self {
        Self { width, height }
    }

    pub fn is_legal_position(&self, object: &Object) -> bool {
        if object.y < 0 || object.y >= self.width {
            return false;
        }

        if object.x < 0 || object.x >= self.height {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::{direction::Direction, parser::parse_header};

    use super::*;

    #[test]
    fn returns_false_when_out_of_bounds() {
        // Moving North
        let (board, mut object) = parse_header("2,2,1,1");
        assert!(object.move_forward(&board));
        assert_eq!(object.y, 0);

        assert!(!object.move_forward(&board));

        // Moving south
        let (board, mut object) = parse_header("2,2,1,1");
        assert!(!object.move_backward(&board));

        // Moving West
        let (board, mut object) = parse_header("2,2,1,1");
        object.direction = Direction::West;

        assert!(object.move_forward(&board));
        assert_eq!(object.x, 0);

        assert!(!object.move_forward(&board));

        // Moving East
        let (board, mut object) = parse_header("2,2,1,1");
        object.direction = Direction::East;

        assert!(!object.move_forward(&board));
    }
}
