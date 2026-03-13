use anyhow::{Result, anyhow, bail};
use std::{collections::HashSet, fmt::Display, rc::Rc, str::FromStr};

use crate::{Direction, Grid, Position, Rank, concepts::distance::manhattan_sum};

const EMPTY_SYMBOL: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub cost_so_far: usize,
    pub move_counter: usize,
    pub rank: Rank,
    pub previous: Option<Rc<State>>,
    pub previous_move: Option<Direction>,
    grid: Grid,
    empty_pos: Position,
}

impl State {
    pub fn new(matrix: Vec<Vec<u8>>) -> Result<Self> {
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
            cost_so_far: 0,
            move_counter: 0,
            rank: inferred_rank,
            previous: None,
            previous_move: None,
            grid: matrix.into(),
            empty_pos,
        })
    }

    pub fn move_empty_tile_to(&self, direction: Direction) -> Option<Self> {
        let new_pos = self.try_move(self.empty_pos, direction)?;
        let new_grid = self.grid.swap_values(&self.empty_pos, &new_pos);

        Some(Self {
            cost_so_far: self.move_counter + manhattan_sum(&new_grid),
            move_counter: self.move_counter + 1,
            rank: self.rank,
            previous: Some(Rc::new(self.clone())),
            previous_move: Some(direction),
            grid: new_grid,
            empty_pos: new_pos,
        })
    }

    pub fn is_solved(&self) -> bool {
        self.grid == *self.rank.get_solved()
    }

    pub fn is_solvable(&self) -> bool {
        let inversions = self.grid.count_inversions();

        let is_even_rank = self.rank.is_even();
        let has_even_inversions = inversions.is_multiple_of(2);
        let is_empty_tile_on_even_row = self.empty_pos.y.is_multiple_of(2);

        if !is_even_rank {
            return has_even_inversions;
        }

        is_empty_tile_on_even_row != has_even_inversions
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

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.grid)?;
        writeln!(f, "Rank: {}", &self.rank)?;
        writeln!(f, "Cost so far: {}", &self.cost_so_far)?;
        writeln!(f, "Empty tile: {}", &self.empty_pos)?;
        write!(f, "Move counter: {}", &self.move_counter)?;

        std::result::Result::Ok(())
    }
}

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matrix = s
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|c| c.parse::<u8>().map_err(|_| anyhow!("Invalid character")))
                    .collect::<Result<Vec<u8>>>()
            })
            .collect::<Result<Vec<Vec<u8>>>>()?;

        State::new(matrix)
    }
}

fn extract_empty_tile(matrix: &[Vec<u8>], rank: Rank) -> Result<Position> {
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

fn is_any_empty(matrix: &[Vec<u8>]) -> bool {
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

fn is_square(matrix: &[Vec<u8>]) -> bool {
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
fn has_valid_cells(matrix: &[Vec<u8>]) -> bool {
    let flat_size = matrix.iter().map(|r| r.len()).sum::<usize>();
    let mut seen = HashSet::with_capacity(flat_size);

    for cell in matrix.iter().flatten() {
        let idx = match usize::from(*cell) {
            i if i < flat_size => i,
            _ => return false,
        };
        if !seen.insert(idx) {
            return false;
        }
    }

    seen.len() == flat_size
}

fn rank_matches_matrix(matrix: &[Vec<u8>], rank: usize) -> Result<bool> {
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
            let state = State::from_str("0 1 2\n3 4 5\n6 7 8").unwrap();
            assert_eq!(
                state.grid.as_matrix(),
                vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
            );
            assert_eq!(state.empty_pos, Position::from((0, 0)));
        }

        #[test]
        fn puzzle_state_from_string() {
            let data: String = String::from("6 7 8\n3 4 5\n0 1 2");
            let state = State::from_str(&data).unwrap();
            assert_eq!(
                state.grid.as_matrix(),
                vec![vec![6, 7, 8], vec![3, 4, 5], vec![0, 1, 2]]
            );
            assert_eq!(state.empty_pos, Position::from((0, 2)));
        }

        #[test]
        fn puzzle_state_rank_4() {
            let data = ["0 1 2 3", "4 5 6 7", "8 9 10 11", "12 13 14 15"].join("\n");

            let state = State::from_str(&data).unwrap();
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
            let state = State::from_str("");
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
            .map(|d| State::from_str(d))
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
            .map(|d| State::from_str(d))
            .iter()
            .for_each(|res| assert!(res.is_err()));
        }
    }

    mod actions {
        use super::*;

        #[test]
        fn move_empty_tile() {
            let state = State::from_str("0 1 2\n3 4 5\n6 7 8").unwrap();

            let mut new_state = state
                .move_empty_tile_to(Direction::Down)
                .expect("Failed to move");

            assert_eq!(new_state.empty_pos, Position::new(0, 1));
            assert_eq!(new_state.grid.at(Position::new(0, 0)), 3);
            assert_eq!(new_state.move_counter, 1);

            new_state = new_state
                .move_empty_tile_to(Direction::Right)
                .expect("Failed to move");
            assert_eq!(new_state.empty_pos, Position::new(1, 1));
            assert_eq!(new_state.grid.at(Position::new(0, 1)), 4);
            assert_eq!(new_state.move_counter, 2);

            new_state = new_state
                .move_empty_tile_to(Direction::Right)
                .expect("Failed to move");
            assert_eq!(new_state.empty_pos, Position::new(2, 1));
            assert_eq!(new_state.grid.at(Position::new(1, 1)), 5);
            assert_eq!(new_state.move_counter, 3);
        }

        #[test]
        fn out_of_bounds_move_returns_none() {
            let state = State::from_str("0 1 2\n3 4 5\n6 7 8").unwrap();

            let mut new_state = state.move_empty_tile_to(Direction::Left);
            assert_eq!(state.empty_pos, Position::new(0, 0));
            assert_eq!(state.move_counter, 0);
            assert_eq!(new_state, None);

            new_state = state.move_empty_tile_to(Direction::Up);
            assert_eq!(state.empty_pos, Position::new(0, 0));
            assert_eq!(state.move_counter, 0);
            assert_eq!(new_state, None)
        }
    }

    #[cfg(test)]
    mod is_solved {
        use super::*;

        #[test]
        fn should_be_true() {
            let state = State::from_str("1 2 3\n4 5 6\n7 8 0").unwrap();
            assert!(state.is_solved())
        }

        #[test]
        fn should_be_false() {
            let state = State::from_str("1 2 3\n4 5 6\n7 0 8").unwrap();
            assert_eq!(state.is_solved(), false)
        }

        #[test]
        fn is_true_after_move() {
            let state = State::from_str("1 2 3\n4 5 6\n7 0 8").unwrap();

            let moved = state.move_empty_tile_to(Direction::Right);
            assert_eq!(state.is_solved(), false);
            assert!(moved.is_some());
            assert!(moved.unwrap().is_solved())
        }
    }

    #[cfg(test)]
    mod solveable {
        use super::*;

        #[test]
        fn should_be_solveable() {
            let state = State::new(vec![
                vec![6, 13, 7, 10],
                vec![8, 9, 11, 0],
                vec![15, 2, 12, 5],
                vec![14, 3, 1, 4],
            ])
            .unwrap();

            assert!(state.is_solvable());
        }

        #[test]
        fn should_not_be_solveable() {
            let state = State::new(vec![
                vec![3, 9, 1, 15],
                vec![14, 11, 4, 6],
                vec![13, 0, 10, 12],
                vec![2, 7, 8, 5],
            ])
            .unwrap();

            assert_eq!(state.is_solvable(), false);
        }
    }

    #[cfg(test)]
    mod heuristics {
        use super::*;

        #[test]
        fn manhattan_sum_simple() {
            let unsolved = State::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 0, 8]]).unwrap();
            assert_eq!(manhattan_sum(&unsolved.grid), 2);

            let solved = unsolved.move_empty_tile_to(Direction::Right).unwrap();
            assert_eq!(manhattan_sum(&solved.grid), 0);
        }
    }
}
