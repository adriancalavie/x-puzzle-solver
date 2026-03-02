use std::{fmt::Display, str::FromStr};

use anyhow::{Context, Result, bail};
use num::{Num, ToPrimitive};

use crate::puzzle_state::PuzzleState;

pub(crate) mod distance;
pub(crate) mod parser;
pub(crate) mod point;
pub(crate) mod puzzle_state;
pub(crate) mod utils;

pub struct Puzzle {
    pub state: PuzzleState,
    pub previous_states: Vec<PuzzleState>,
    pub rank: u8,
}

impl Puzzle {
    pub fn new(state: PuzzleState, rank: u8) -> Result<Self> {
        Self::create(state, rank)
    }

    pub fn from_matrix(matrix: Vec<Vec<u32>>, rank: u8) -> Result<Self> {
        Self::new(PuzzleState::new(matrix)?, rank)
    }

    pub fn from_str(matrix: &str, rank: u8) -> Result<Self> {
        Self::new(PuzzleState::from_str(matrix)?, rank)
    }

    fn create(state: PuzzleState, rank: u8) -> Result<Self> {
        if !rank_matches_matrix(&state.matrix, rank)? {
            bail!("Rank does not match matrix shape.");
        }

        Ok(Self {
            state,
            previous_states: Vec::new(),
            rank,
        })
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix:")?;
        writeln!(f, "{}", self.state)?;
        write!(f, "Rank: {}", self.rank)?;

        Ok(())
    }
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
