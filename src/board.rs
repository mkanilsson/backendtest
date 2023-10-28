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
