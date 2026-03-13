use std::fmt::Display;

use strum_macros::EnumIter;

use crate::Offset;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "↑"),
            Direction::Down => write!(f, "↓"),
            Direction::Left => write!(f, "←"),
            Direction::Right => write!(f, "→"),
        }
    }
}
