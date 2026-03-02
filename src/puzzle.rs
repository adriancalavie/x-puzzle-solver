use std::{collections::HashSet, str::FromStr};

use num::{Num, ToPrimitive};

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
    pub fn new(matrix: Vec<Vec<u32>>) -> Self {
        Self {
            matrix,
            cost_so_far: 0,
            move_counter: 0,
        }
    }
}

impl FromStr for PuzzleState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matrix = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .ok_or_else(|| "Invalid character".to_string())
                    })
                    .collect::<Result<Vec<u32>, _>>()
            })
            .collect::<Result<Vec<Vec<u32>>, _>>()?;

        if !is_square(&matrix) {
            return Err("Matrix should be square shaped".to_string());
        }

        if !has_valid_cells(&matrix) {
            return Err("Matrix should have values between 0 and (rank^2)-1".to_string());
        }

        Ok(Self::new(matrix))
    }
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
    use super::*;

    #[test]
    fn puzzle_state_from_string() {
        let data = "123\n456\n789";
        let state = PuzzleState::from_str(data).unwrap();
        assert_eq!(
            state.matrix,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        );
    }
}
