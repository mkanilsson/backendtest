#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_clockwise() {
        let mut direction = Direction::North;
        direction.rotate_clockwise();
        assert_eq!(direction, Direction::East);
        direction.rotate_clockwise();
        assert_eq!(direction, Direction::South);
        direction.rotate_clockwise();
        assert_eq!(direction, Direction::West);
        direction.rotate_clockwise();
        assert_eq!(direction, Direction::North);
    }

    #[test]
    fn rotate_counter_clockwise() {
        let mut direction = Direction::North;
        direction.rotate_counter_clockwise();
        assert_eq!(direction, Direction::West);
        direction.rotate_counter_clockwise();
        assert_eq!(direction, Direction::South);
        direction.rotate_counter_clockwise();
        assert_eq!(direction, Direction::East);
        direction.rotate_counter_clockwise();
        assert_eq!(direction, Direction::North);
    }
}
