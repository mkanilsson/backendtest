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
