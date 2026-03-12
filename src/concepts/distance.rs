use crate::{Grid, Position, Rank};

/// Computes the manhattan distance between two points
pub fn manhattan(p1: Position, p2: Position) -> usize {
    p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
}

pub fn manhattan_sum(grid: &Grid) -> usize {
    let mut sum = 0;

    for (idx, value) in grid.get_data().iter().enumerate() {
        let current = grid.index_to_pos(idx);
        let target = grid.index_to_pos(Rank::solved_idx_for(grid.rank, *value));

        sum += manhattan(current, target)
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance_cases() {
        let cases = [((1, 2), (4, 6), 7), ((3, 5), (2, 1), 5)];

        for &((ax, ay), (bx, by), expected) in &cases {
            let result = manhattan((ax, ay).into(), (bx, by).into());
            assert_eq!(result, expected, "case: ({},{}) vs ({},{})", ax, ay, bx, by);
        }
    }
}
