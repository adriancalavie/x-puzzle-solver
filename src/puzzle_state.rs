use anyhow::{Result, anyhow, bail};
use std::{collections::HashSet, fmt::Display, str::FromStr};

use crate::{Direction, Point, Rank, utils::clamp};

const EMPTY_SYMBOL: i32 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PuzzleState {
    pub cost_so_far: i32,
    pub move_counter: i32,
    pub rank: Rank,
    matrix: Vec<Vec<i32>>,
    empty_pos: Point,
}

impl PuzzleState {
    pub fn new(matrix: Vec<Vec<i32>>) -> Result<Self> {
        if is_any_empty(&matrix) {
            bail!("Matrix is empty or has empty rows.");
        }

        if !is_square(&matrix) {
            bail!("Matrix should be square shaped.");
        }

        if !has_valid_cells(&matrix) {
            bail!("Matrix should have values between 0 and (rank^2)-1");
        }

        let inferred_rank: Rank = matrix
            .len()
            .try_into()
            .map_err(|e| anyhow!("Can't convert to Rank: {e}"))?;

        if !rank_matches_matrix(&matrix, inferred_rank.into())? {
            bail!("Rank does not match matrix shape.");
        }

        let empty_pos = extract_empty_tile(&matrix, inferred_rank)?;

        Ok(Self {
            matrix,
            cost_so_far: 0,
            move_counter: 0,
            rank: inferred_rank,
            empty_pos,
        })
    }

    pub fn move_empty_tile_to(&mut self, direction: Direction) {
        let new_tile_pos = self.in_bounds(self.empty_pos.moved(direction));

        if new_tile_pos.eq(&self.empty_pos) {
            return;
        }
    }

    fn in_bounds(&self, pos: Point) -> Point {
        Point {
            x: clamp(pos.x, 0, i32::from(self.rank) - 1),
            y: clamp(pos.y, 0, i32::from(self.rank) - 1),
        }
    }
}

impl Display for PuzzleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.matrix {
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Rank: {}", &self.rank)?;
        writeln!(f, "Cost so far: {}", &self.cost_so_far)?;
        writeln!(f, "Empty tile: {}", &self.empty_pos)?;
        write!(f, "Move counter: {}", &self.move_counter)?;

        std::result::Result::Ok(())
    }
}

impl FromStr for PuzzleState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matrix = s
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|c| c.parse::<i32>().map_err(|_| anyhow!("Invalid character")))
                    .collect::<Result<Vec<i32>>>()
            })
            .collect::<Result<Vec<Vec<i32>>>>()?;

        PuzzleState::new(matrix)
    }
}

fn extract_empty_tile(matrix: &[Vec<i32>], rank: Rank) -> Result<Point> {
    let rank_usize = usize::from(rank);

    matrix
        .iter()
        .flatten()
        .enumerate()
        .find_map(|(idx, cell)| {
            if *cell == EMPTY_SYMBOL {
                let x = idx % rank_usize;
                let y = idx / rank_usize;
                Some((x, y))
            } else {
                None
            }
        })
        .map(|(x, y)| {
            let x = i32::try_from(x).map_err(|_| anyhow!("X too large for i32"))?;
            let y = i32::try_from(y).map_err(|_| anyhow!("Y too large for i32"))?;
            anyhow::Ok(Point::new(x, y))
        })
        .transpose()?
        .ok_or_else(|| anyhow!("Empty tile couldn't be found in the matrix"))
}

fn is_any_empty(matrix: &[Vec<i32>]) -> bool {
    if matrix.is_empty() {
        return true;
    }
    for line in matrix {
        if line.is_empty() {
            return true;
        }
    }

    false
}

fn is_square(matrix: &[Vec<i32>]) -> bool {
    let lines_count = matrix.len();
    for line in matrix {
        if line.len() != lines_count {
            return false;
        }
    }

    true
}

/// Checks if:
///  - all elements are between zero and (flat_matrix_size - 1)
///  - appear at most once (actually, only once)
fn has_valid_cells(matrix: &[Vec<i32>]) -> bool {
    let flat_size = matrix.iter().map(|r| r.len()).sum::<usize>();
    let mut seen = HashSet::with_capacity(flat_size);

    for cell in matrix.iter().flatten() {
        let idx = match usize::try_from(*cell) {
            Ok(i) if i < flat_size => i,
            _ => return false,
        };
        if !seen.insert(idx) {
            return false;
        }
    }

    seen.len() == flat_size
}

fn rank_matches_matrix(matrix: &[Vec<i32>], rank: usize) -> Result<bool> {
    let row_count = matrix.len();
    let col_count = matrix.first().unwrap().len();

    if row_count != rank || col_count != rank {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Point;

    use super::PuzzleState;

    #[test]
    fn puzzle_state_from_str() {
        let state = PuzzleState::from_str("0 1 2\n3 4 5\n6 7 8").unwrap();
        assert_eq!(
            state.matrix,
            vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
        assert_eq!(state.empty_pos, Point::from((0, 0)));
    }

    #[test]
    fn puzzle_state_from_string() {
        let data: String = String::from("6 7 8\n3 4 5\n0 1 2");
        let state = PuzzleState::from_str(&data).unwrap();
        assert_eq!(
            state.matrix,
            vec![vec![6, 7, 8], vec![3, 4, 5], vec![0, 1, 2]]
        );
        assert_eq!(state.empty_pos, Point::from((0, 2)));
    }

    #[test]
    fn puzzle_state_rank_4() {
        let data = ["0 1 2 3", "4 5 6 7", "8 9 10 11", "12 13 14 15"].join("\n");

        let state = PuzzleState::from_str(&data).unwrap();
        assert_eq!(
            state.matrix,
            vec![
                vec![0, 1, 2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11],
                vec![12, 13, 14, 15]
            ]
        );
        assert_eq!(state.empty_pos, Point::from((0, 0)));
    }

    #[test]
    fn expect_err_if_input_empty() {
        let state = PuzzleState::from_str("");
        assert!(state.is_err_and(|e| e.to_string().eq("Matrix is empty or has empty rows.")));
    }

    #[test]
    fn expect_err_if_input_not_square() {
        [
            "0 1 2\n3 4 5\n6 7 8 9",
            "0 1 2\n3 4 5\n6 7",
            "0 1 2\n3 4 5",
            "0 1 2\n3 4 5\n6 7 8 \n9",
        ]
        .map(|d| PuzzleState::from_str(d))
        .iter()
        .for_each(|res| assert!(res.is_err()));
    }

    #[test]
    fn expect_err_if_cells_not_valid() {
        [
            "0 0 0\n0 0 0\n0 0 0",
            "0 1 2\n3 4 5\n6 7 9",
            "1 2 3\n4 5 6\n7 8 9",
            "14 5 6\n4 5 6\n7 8 9",
        ]
        .map(|d| PuzzleState::from_str(d))
        .iter()
        .for_each(|res| assert!(res.is_err()));
    }
}
