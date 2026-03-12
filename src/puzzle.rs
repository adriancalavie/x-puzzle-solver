use anyhow::Result;
use std::{fmt::Display, str::FromStr};

use crate::{PuzzleState, Rank};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Puzzle {
    pub state: PuzzleState,
    pub previous_states: Vec<PuzzleState>,
}

impl Puzzle {
    fn create(state: PuzzleState) -> Result<Self> {
        Ok(Self {
            state,
            previous_states: Vec::new(),
        })
    }

    pub fn new(state: PuzzleState) -> Result<Self> {
        Self::create(state)
    }

    pub fn from_matrix(matrix: Vec<Vec<i32>>) -> Result<Self> {
        Self::new(PuzzleState::new(matrix)?)
    }

    pub fn from_str(matrix: &str) -> Result<Self> {
        Self::new(PuzzleState::from_str(matrix)?)
    }

    pub fn get_rank(&self) -> Rank {
        self.state.rank
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix:")?;
        write!(f, "{}", self.state)?;

        Ok(())
    }
}
