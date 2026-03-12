use crate::Offset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn offset(&self) -> Offset {
        match self {
            Direction::Up => Offset { dx: 0, dy: -1 },
            Direction::Down => Offset { dx: 0, dy: 1 },
            Direction::Left => Offset { dx: -1, dy: 0 },
            Direction::Right => Offset { dx: 1, dy: 0 },
        }
    }
}
