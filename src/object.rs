pub struct Object {
    pub x: usize,
    pub y: usize,
}

impl Object {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
