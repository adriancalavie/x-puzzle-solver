use crate::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn value(&self) -> Point {
        match self {
            Direction::UP => Point { x: 0, y: 1 },
            Direction::DOWN => Point { x: 0, y: -1 },
            Direction::LEFT => Point { x: -1, y: 0 },
            Direction::RIGHT => Point { x: 1, y: 0 },
        }
    }
}
