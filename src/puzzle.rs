use anyhow::{Result, anyhow, bail};
use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    hash::Hash,
    rc::Rc,
    str::FromStr,
};
use strum::IntoEnumIterator;

use crate::{Direction, Rank, State};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Puzzle {
    pub initial: State,
}

impl Puzzle {
    fn create(state: State) -> Result<Self> {
        Ok(Self { initial: state })
    }

    pub fn new(state: State) -> Result<Self> {
        Self::create(state)
    }

    pub fn from_matrix(matrix: Vec<Vec<u8>>) -> Result<Self> {
        Self::new(State::new(matrix)?)
    }

    pub fn get_rank(&self) -> Rank {
        self.initial.rank
    }

    pub fn is_solvable(&self) -> bool {
        self.initial.is_solvable()
    }

    pub fn solve(&self) -> Result<State> {
        if !self.initial.is_solvable() {
            return Err(anyhow!("initial state is not solvable"));
        }

        if self.initial.is_solved() {
            return Ok(self.initial.clone());
        }

        let mut frontier = BinaryHeap::new();
        let mut checked = HashSet::new();

        frontier.push(self.initial.clone());

        while let Some(current) = frontier.pop() {
            if current.is_solved() {
                return Ok(current);
            }
            if !checked.insert(current.grid().clone()) {
                continue;
            }

            for direction in Direction::iter() {
                if let Some(next) = current.move_empty_tile_to(direction)
                    && !checked.contains(next.grid())
                {
                    frontier.push(next);
                }
            }
        }

        bail!("no solution found");
    }

    pub fn print_solved(state: State) {
        let states = Self::get_states_chronologically(state);
        for state in states {
            println!("{}", state);
        }
    }

    pub fn print_moves_to_solve(state: State) {
        let states = Self::get_states_chronologically(state);
        for (idx, state) in states.iter().enumerate() {
            if let Some(direction) = state.previous_move {
                println!("Move no {}: {}", idx, direction);
            }
        }
    }

    fn get_states_chronologically(state: State) -> Vec<Rc<State>> {
        let mut states = Vec::new();
        let mut current = Rc::new(state);
        states.insert(0, current.clone());
        while let Some(parent) = &current.previous {
            current = parent.clone();
            states.insert(0, current.clone());
        }

        states
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix:")?;
        write!(f, "{}", self.initial)?;

        Ok(())
    }
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(string_matrix: &str) -> Result<Self, Self::Err> {
        Self::new(State::from_str(string_matrix)?)
    }
}
