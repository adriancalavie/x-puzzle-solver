use num::{Signed, abs};

use crate::point::Point;

pub fn manhattan_distance<T>(p1: Point<T>, p2: Point<T>) -> T
where
    T: Signed,
{
    let (x1, y1) = p1.into();
    let (x2, y2) = p2.into();

    abs(x1 - x2) + abs(y1 - y2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance_cases() {
        let cases = [((1, 2), (4, 6), 7), ((-3, 5), (2, -1), 11)];

        for &((ax, ay), (bx, by), expected) in &cases {
            let result = manhattan_distance((ax, ay).into(), (bx, by).into());
            assert_eq!(result, expected, "case: ({},{}) vs ({},{})", ax, ay, bx, by);
        }
    }
}
