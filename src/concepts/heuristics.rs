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
    let goal_position = |v| grid.index_to_pos(Rank::solved_idx_for(grid.rank, v));

    let rows = grid.as_rows();
    let mut conflicts = 0;

    for (row_idx, row) in rows.iter().enumerate() {
        let belong_in_this_row: Vec<&u8> = row
            .iter()
            .filter(|v| **v != 0 && goal_position(**v).y == row_idx)
            .collect();
        if belong_in_this_row.len() < 2 {
            continue;
        }
        for i in 0..belong_in_this_row.len() - 1 {
            let ix = goal_position(*belong_in_this_row[i]).x;
            for j in i + 1..belong_in_this_row.len() {
                let jx = goal_position(*belong_in_this_row[j]).x;
                if jx < ix {
                    conflicts += 1;
                }
            }
        }
    }

    conflicts
}

fn linear_conflicts_by_col(grid: &Grid) -> usize {
    let goal_position = |v| grid.index_to_pos(Rank::solved_idx_for(grid.rank, v));

    let cols = grid.as_cols();
    let mut conflicts = 0;

    for (col_idx, col) in cols.iter().enumerate() {
        let belong_in_this_col: Vec<&u8> = col
            .iter()
            .filter(|v| **v != 0 && goal_position(**v).x == col_idx)
            .collect();
        if belong_in_this_col.len() < 2 {
            continue;
        }
        for i in 0..belong_in_this_col.len() - 1 {
            let iy = goal_position(*belong_in_this_col[i]).y;
            for j in i + 1..belong_in_this_col.len() {
                let jy = goal_position(*belong_in_this_col[j]).y;
                if jy < iy {
                    conflicts += 1;
                }
            }
        }
    }

    conflicts
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
