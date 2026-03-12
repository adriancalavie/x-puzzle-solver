use anyhow::Result;
use std::{fmt::Display, str::FromStr};

use crate::{Rank, State};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Puzzle {
    pub state: State,
    pub previous_states: Vec<State>,
}

impl Puzzle {
    fn create(state: State) -> Result<Self> {
        Ok(Self {
            state,
            previous_states: Vec::new(),
        })
    }

    pub fn new(state: State) -> Result<Self> {
        Self::create(state)
    }

    pub fn from_matrix(matrix: Vec<Vec<i32>>) -> Result<Self> {
        Self::new(State::new(matrix)?)
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

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(string_matrix: &str) -> Result<Self, Self::Err> {
        Self::new(State::from_str(string_matrix)?)
    }
}
