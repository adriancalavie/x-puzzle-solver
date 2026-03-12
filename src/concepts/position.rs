use std::{fmt::Display, ops::Add};

use crate::{Direction, Offset};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn moved(self, direction: Direction) -> Option<Self> {
        self + direction.offset()
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::zero()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)?;

        std::result::Result::Ok(())
    }
}

impl Add<Offset> for Position {
    type Output = Option<Self>;

    fn add(self, offset: Offset) -> Option<Self> {
        let x = (self.x as i32).checked_add(offset.dx)?;
        let y = (self.y as i32).checked_add(offset.dy)?;
        if x < 0 || y < 0 {
            return None;
        }
        Some(Position {
            x: x as usize,
            y: y as usize,
        })
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<Position> for (usize, usize) {
    fn from(p: Position) -> Self {
        (p.x, p.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point_from_tuple() {
        let created = Position::from((1, 2));
        assert_eq!(created.x, 1);
        assert_eq!(created.y, 2);
    }
}
