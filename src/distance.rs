use crate::point::Point;

/// Computes the manhattan distance between two points
pub fn manhattan_distance(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
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
