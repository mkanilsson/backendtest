#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    MoveForward,
    MoveBackwards,
    RotateClockwise,
    RotateCounterclockwise,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value {
            "0" => Command::Quit,
            "1" => Command::MoveForward,
            "2" => Command::MoveBackwards,
            "3" => Command::RotateClockwise,
            "4" => Command::RotateCounterclockwise,
            other => panic!("Unknown command {}", other),
        }
    }
}
