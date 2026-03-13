use crate::{Grid, Rank, concepts::distance::manhattan};

pub fn manhattan_sum(grid: &Grid) -> usize {
    let mut sum = 0;

    for (idx, value) in grid.get_data().iter().enumerate() {
        let current = grid.index_to_pos(idx);
        let target = grid.index_to_pos(Rank::solved_idx_for(grid.rank, *value));

        sum += manhattan(current, target)
    }
    sum
}

pub fn linear_conflicts(grid: &Grid) -> usize {
    linear_conflicts_by_row(grid) + linear_conflicts_by_col(grid)
}

fn linear_conflicts_by_row(grid: &Grid) -> usize {
    let goal_position = |v: &u8| grid.index_to_pos(Rank::solved_idx_for(grid.rank, *v));
    grid.as_rows()
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let belong: Vec<&u8> = row
                .iter()
                .filter(|v| **v != 0 && goal_position(v).y == row_idx)
                .collect();
            count_conflicts_in_line(&belong, |v| goal_position(v).x)
        })
        .sum()
}

fn linear_conflicts_by_col(grid: &Grid) -> usize {
    let goal_position = |v: &u8| grid.index_to_pos(Rank::solved_idx_for(grid.rank, *v));
    grid.as_cols()
        .iter()
        .enumerate()
        .map(|(col_idx, col)| {
            let belong: Vec<&u8> = col
                .iter()
                .filter(|v| **v != 0 && goal_position(v).x == col_idx)
                .collect();
            count_conflicts_in_line(&belong, |v| goal_position(v).y)
        })
        .sum()
}

fn count_conflicts_in_line(line: &[&u8], goal_axis: impl Fn(&u8) -> usize) -> usize {
    let positions: Vec<usize> = line.iter().map(|v| goal_axis(v)).collect();
    positions
        .iter()
        .enumerate()
        .flat_map(|(i, &pos_i)| {
            positions[i + 1..]
                .iter()
                .filter(move |&&pos_j| pos_j < pos_i)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_linear_conflicts() {
        let grid = Grid::new(vec![3, 2, 1, 5, 4, 6, 7, 8, 0], 3);

        assert_eq!(linear_conflicts_by_row(&grid), 4);
    }

    #[test]
    fn col_linear_conflicts() {
        let grid = Grid::new(vec![1, 8, 3, 4, 2, 0, 7, 5, 6], 3);

        assert_eq!(linear_conflicts_by_col(&grid), 2);
    }
}
