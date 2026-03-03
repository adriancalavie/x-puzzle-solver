use anyhow::{Context, Ok, Result, anyhow, bail};
use num::{Num, ToPrimitive};
use std::{collections::HashSet, fmt::Display, str::FromStr};

pub struct PuzzleState {
    pub matrix: Vec<Vec<u32>>,
    pub cost_so_far: u16,
    pub move_counter: u16,
    pub rank: u8,
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

        let inferred_rank = matrix
            .len()
            .to_u8()
            .ok_or_else(|| anyhow!("Can't convert matrix size to u8"))?;

        if !rank_matches_matrix(&matrix, inferred_rank)? {
            bail!("Rank does not match matrix shape.");
        }

        Ok(Self {
            matrix,
            cost_so_far: 0,
            move_counter: 0,
            rank: inferred_rank,
        })
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
                    .map(|c| {
                        c.parse::<u32>()
                            .map_err(|_| anyhow::anyhow!("Invalid character"))
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

/// Checks if:
///  - all elements are between zero and (flat_matrix_size - 1)
///  - appear at most once (actually, only once)
fn has_valid_cells<T>(matrix: &[Vec<T>]) -> bool
where
    T: Num + PartialOrd + ToPrimitive,
{
    let flat_size = matrix.iter().map(|r| r.len()).sum::<usize>();
    let mut seen = HashSet::with_capacity(flat_size);

    for v in matrix.iter().flatten() {
        let idx = match v.to_usize() {
            Some(i) if i < flat_size => i,
            _ => return false,
        };
        if !seen.insert(idx) {
            return false; // duplicate
        }
    }

    seen.len() == flat_size
}

fn rank_matches_matrix<T>(matrix: &[Vec<T>], rank: u8) -> Result<bool>
where
    T: Num,
{
    let row_count = matrix.len();
    let col_count = matrix.first().unwrap().len();

    let rank_usize = rank.to_usize().with_context(|| "Rank too high")?;

    if row_count != rank_usize || col_count != rank_usize {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::PuzzleState;

    #[test]
    fn puzzle_state_from_str() {
        let state = PuzzleState::from_str("0 1 2\n3 4 5\n6 7 8").unwrap();
        assert_eq!(
            state.matrix,
            vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
    }

    #[test]
    fn puzzle_state_from_string() {
        let data: String = String::from("6 7 8\n3 4 5\n0 1 2");
        let state = PuzzleState::from_str(&data).unwrap();
        assert_eq!(
            state.matrix,
            vec![vec![6, 7, 8], vec![3, 4, 5], vec![0, 1, 2]]
        );
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
