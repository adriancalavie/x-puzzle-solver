use anyhow::{Result, bail};
use num::{Num, ToPrimitive};
use std::{collections::HashSet, str::FromStr};

pub struct Puzzle {
    pub state: PuzzleState,
    pub previous_states: Vec<PuzzleState>,
    pub rank: u8,
}

impl Puzzle {
    pub fn new(state: PuzzleState, rank: u8) -> Self {
        Self {
            state,
            previous_states: Vec::new(),
            rank,
        }
    }
}

pub struct PuzzleState {
    pub matrix: Vec<Vec<u32>>,
    pub cost_so_far: u16,
    pub move_counter: u16,
}

impl PuzzleState {
    pub fn new(matrix: Vec<Vec<u32>>) -> Result<Self> {
        if is_any_empty(&matrix) {
            bail!("Matrix is empty or has empty rows.");
        }

        if !is_square(&matrix) {
            bail!("Matrix should be square shaped.");
        }

        if !has_valid_cells(&matrix) {
            bail!("Matrix should have values between 0 and (rank^2)-1");
        }

        Ok(Self {
            matrix,
            cost_so_far: 0,
            move_counter: 0,
        })
    }
}

impl FromStr for PuzzleState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matrix = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .ok_or_else(|| anyhow::anyhow!("Invalid character"))
                    })
                    .collect::<Result<Vec<u32>>>()
            })
            .collect::<Result<Vec<Vec<u32>>>>()?;

        PuzzleState::new(matrix)
    }
}

fn is_any_empty<T>(matrix: &[Vec<T>]) -> bool
where
    T: Num,
{
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

fn is_square<T>(matrix: &[Vec<T>]) -> bool
where
    T: Num,
{
    let lines_count = matrix.len();
    for line in matrix {
        if line.len() != lines_count {
            return false;
        }
    }

    true
}

fn has_valid_cells<T>(matrix: &[Vec<T>]) -> bool
where
    T: Num + PartialOrd + ToPrimitive,
{
    let total = matrix.iter().map(|r| r.len()).sum::<usize>();
    let mut seen = HashSet::with_capacity(total);

    for v in matrix.iter().flatten() {
        let idx = match v.to_usize() {
            Some(i) if i < total => i,
            _ => return false,
        };
        if !seen.insert(idx) {
            return false; // duplicate
        }
    }

    seen.len() == total
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::PuzzleState;

    #[test]
    fn puzzle_state_from_str() {
        let state = PuzzleState::from_str("012\n345\n678").unwrap();
        assert_eq!(
            state.matrix,
            vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
    }

    #[test]
    fn puzzle_state_from_string() {
        let data: String = String::from("n678\n345\n012");
        let state = PuzzleState::from_str(&data).unwrap();
        assert_eq!(
            state.matrix,
            vec![vec![6, 7, 8], vec![3, 4, 5], vec![0, 1, 2]]
        );
    }

    #[test]
    fn expect_err_if_input_empty() {
        let state = PuzzleState::from_str("");
        assert!(state.is_err_and(|e| e.to_string().eq("Matrix is empty or has empty rows.")));
    }

    #[test]
    fn expect_err_if_input_not_square() {
        [
            "012\n345\n6789",
            "012\n345\n67",
            "012\n345",
            "012\n345\n678\n9",
        ]
        .map(|d| PuzzleState::from_str(d))
        .iter()
        .for_each(|res| assert!(res.is_err()));
    }

    #[test]
    fn expect_err_if_cells_not_valid() {
        [
            "000\n000\n000",
            "012\n345\n679",
            "123\n456\n789",
            "456\n456\n789",
        ]
        .map(|d| PuzzleState::from_str(d))
        .iter()
        .for_each(|res| assert!(res.is_err()));
    }
}
