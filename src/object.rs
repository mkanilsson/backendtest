use crate::direction::Direction;

pub struct Object {
    pub x: usize,
    pub y: usize,

    pub direction: Direction,
}

impl Object {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::North,
        }
    }
}
