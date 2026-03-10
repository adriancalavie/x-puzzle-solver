use crate::Offset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn offset(&self) -> Offset {
        match self {
            Direction::UP => Offset { dx: 0, dy: -1 },
            Direction::DOWN => Offset { dx: 0, dy: 1 },
            Direction::LEFT => Offset { dx: -1, dy: 0 },
            Direction::RIGHT => Offset { dx: 1, dy: 0 },
        }
    }
}
