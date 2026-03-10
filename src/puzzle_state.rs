use anyhow::{Result, anyhow, bail};
use std::{collections::HashSet, fmt::Display, str::FromStr};

use crate::{Direction, Grid, Position, Rank};

const EMPTY_SYMBOL: i32 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PuzzleState {
    pub cost_so_far: i32,
    pub move_counter: usize,
    pub rank: Rank,
    grid: Grid<i32>,
    empty_pos: Position,
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
            grid: matrix.into(),
            cost_so_far: 0,
            move_counter: 0,
            rank: inferred_rank,
            empty_pos,
        })
    }

    pub fn move_empty_tile_to(&mut self, direction: Direction) {
        let Some(new_pos) = self.try_move(self.empty_pos, direction) else {
            return;
        };
        self.grid.swap_values(&self.empty_pos, &new_pos);
        self.empty_pos = new_pos;
        self.move_counter += 1;
    }

    pub fn is_solved(&self) -> bool {
        self.grid == *self.rank.get_solved()
    }

    fn try_move(&self, pos: Position, dir: Direction) -> Option<Position> {
        let new_pos = (pos + dir.offset())?;
        let rank = self.rank.into();
        if new_pos.x < rank && new_pos.y < rank {
            Some(new_pos)
        } else {
            None
        }
    }
}

impl Display for PuzzleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.grid)?;
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

fn extract_empty_tile(matrix: &[Vec<i32>], rank: Rank) -> Result<Position> {
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
        .map(|(x, y)| anyhow::Ok(Position::new(x, y)))
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
    use super::*;

    mod creation_and_validation {
        use super::*;

        #[test]
        fn puzzle_state_from_str() {
            let state = PuzzleState::from_str("0 1 2\n3 4 5\n6 7 8").unwrap();
            assert_eq!(
                state.grid.as_matrix(),
                vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
            );
            assert_eq!(state.empty_pos, Position::from((0, 0)));
        }

        #[test]
        fn puzzle_state_from_string() {
            let data: String = String::from("6 7 8\n3 4 5\n0 1 2");
            let state = PuzzleState::from_str(&data).unwrap();
            assert_eq!(
                state.grid.as_matrix(),
                vec![vec![6, 7, 8], vec![3, 4, 5], vec![0, 1, 2]]
            );
            assert_eq!(state.empty_pos, Position::from((0, 2)));
        }

        #[test]
        fn puzzle_state_rank_4() {
            let data = ["0 1 2 3", "4 5 6 7", "8 9 10 11", "12 13 14 15"].join("\n");

            let state = PuzzleState::from_str(&data).unwrap();
            assert_eq!(
                state.grid.as_matrix(),
                vec![
                    vec![0, 1, 2, 3],
                    vec![4, 5, 6, 7],
                    vec![8, 9, 10, 11],
                    vec![12, 13, 14, 15]
                ]
            );
            assert_eq!(state.empty_pos, Position::from((0, 0)));
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

    mod actions {
        use super::*;

        #[test]
        fn move_empty_tile() {
            let mut state = PuzzleState::from_str("0 1 2\n3 4 5\n6 7 8").unwrap();

            state.move_empty_tile_to(Direction::DOWN);
            assert_eq!(state.empty_pos, Position::new(0, 1));
            assert_eq!(state.grid.at(Position::new(0, 0)), 3);
            assert_eq!(state.move_counter, 1);

            state.move_empty_tile_to(Direction::RIGHT);
            assert_eq!(state.empty_pos, Position::new(1, 1));
            assert_eq!(state.grid.at(Position::new(0, 1)), 4);
            assert_eq!(state.move_counter, 2);

            state.move_empty_tile_to(Direction::RIGHT);
            assert_eq!(state.empty_pos, Position::new(2, 1));
            assert_eq!(state.grid.at(Position::new(1, 1)), 5);
            assert_eq!(state.move_counter, 3);
        }

        #[test]
        fn out_of_bounds_move_does_nothing() {
            let mut state = PuzzleState::from_str("0 1 2\n3 4 5\n6 7 8").unwrap();

            state.move_empty_tile_to(Direction::LEFT);
            assert_eq!(state.empty_pos, Position::new(0, 0));
            assert_eq!(state.move_counter, 0);

            state.move_empty_tile_to(Direction::UP);
            assert_eq!(state.empty_pos, Position::new(0, 0));
            assert_eq!(state.move_counter, 0);
        }
    }

    #[cfg(test)]
    mod is_solved {
        use super::*;

        #[test]
        fn should_be_true() {
            let state = PuzzleState::from_str("1 2 3\n4 5 6\n7 8 0").unwrap();
            assert!(state.is_solved())
        }

        #[test]
        fn should_be_false() {
            let state = PuzzleState::from_str("1 2 3\n4 5 6\n7 0 8").unwrap();
            assert_eq!(state.is_solved(), false)
        }

        #[test]
        fn is_true_after_move() {
            let mut state = PuzzleState::from_str("1 2 3\n4 5 6\n7 0 8").unwrap();

            state.move_empty_tile_to(Direction::RIGHT);
            assert!(state.is_solved())
        }
    }
}
