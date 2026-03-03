use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn value(&self) -> Point {
        match self {
            Direction::UP => Point { x: 0, y: 1 },
            Direction::DOWN => Point { x: 0, y: -1 },
            Direction::LEFT => Point { x: -1, y: 0 },
            Direction::RIGHT => Point { x: 1, y: 0 },
        }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn move_to(&mut self, direction: Direction) {
        *self += direction.value();
    }

    pub fn moved(self, direction: Direction) -> Self {
        self + direction.value()
    }

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Self;

    fn add(self, other: &Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (i32, i32) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point_from_tuple() {
        let created = Point::from((1, 2));
        assert_eq!(created.x, 1);
        assert_eq!(created.y, 2);
    }

    #[test]
    fn add_two_points() {
        let point1 = Point::new(1, 2);
        let point2 = Point::new(3, 4);

        let point3 = point1 + point2;

        assert_eq!(point3, Point::new(4, 6));
    }

    #[test]
    fn opposite_points_add_to_0_0() {
        let positive = Point::new(1, 1);
        let negative = Point::new(-1, -1);

        assert_eq!(positive + negative, Point::zero())
    }

    #[test]
    fn add_multiple_points() {
        let points = [
            Point::new(1, 2),
            Point::new(3, 4),
            Point::new(5, 6),
            Point::new(7, 8),
        ];
        let sum: Point = points.iter().fold(Point::zero(), |acc, p| acc + p);

        assert_eq!(sum, Point::new(16, 20))
    }
}
