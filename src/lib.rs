use std::{fmt::Display, str::FromStr};

use anyhow::Result;

use crate::puzzle_state::PuzzleState;

pub(crate) mod distance;
pub(crate) mod point;
pub(crate) mod puzzle_state;
pub(crate) mod utils;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl TryFrom<i32> for Rank {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            _ => Err("Rank must be 2, 3, 4, or 5"),
        }
    }
}

impl TryFrom<usize> for Rank {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            _ => Err("Rank must be 2, 3, 4, or 5"),
        }
    }
}

impl From<Rank> for usize {
    fn from(rank: Rank) -> usize {
        rank as usize
    }
}
