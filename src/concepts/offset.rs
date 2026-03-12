#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset {
    pub dx: i32,
    pub dy: i32,
}

impl Offset {
    pub fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }
}
