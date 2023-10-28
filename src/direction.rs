pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn rotate_clockwise(&mut self) {
        *self = match &self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        *self = match &self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}
