use std::{fmt::Display, str::FromStr};

use anyhow::{Context, Result, bail};
use num::{Num, ToPrimitive};

use crate::puzzle_state::PuzzleState;

pub(crate) mod distance;
pub(crate) mod point;
pub(crate) mod puzzle_state;
pub(crate) mod utils;

pub struct Puzzle {
    pub state: PuzzleState,
    pub previous_states: Vec<PuzzleState>,
}

impl Puzzle {
    pub fn new(state: PuzzleState) -> Result<Self> {
        Self::create(state)
    }

    pub fn from_matrix(matrix: Vec<Vec<u32>>) -> Result<Self> {
        Self::new(PuzzleState::new(matrix)?)
    }

    pub fn from_str(matrix: &str) -> Result<Self> {
        Self::new(PuzzleState::from_str(matrix)?)
    }

    fn create(state: PuzzleState) -> Result<Self> {
        Ok(Self {
            state,
            previous_states: Vec::new(),
        })
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix:")?;
        write!(f, "{}", self.state)?;

        Ok(())
    }
}
