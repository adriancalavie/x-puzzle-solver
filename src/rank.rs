use crate::{Position, utils::SOLVED_CACHE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

impl Rank {
    fn compute_solved(&self) -> Vec<u8> {
        let size = usize::from(self).pow(2);
        (1..=size)
            .map(|i| if i == size { 0 } else { i as u8 })
            .collect()
    }

    /// lazy inits, cached reads
    /// See [SOLVED_CACHE](crate::utils::SOLVED_CACHE) for details
    pub fn get_solved(&self) -> &'static [u8] {
        SOLVED_CACHE[self].get_or_init(|| self.compute_solved())
    }

    pub fn get_solved_empty_tile_pos(&self) -> Position {
        let idx_pos = usize::from(self) - 1;
        Position::new(idx_pos, idx_pos)
    }

    pub fn solved_idx_for(rank: usize, value: u8) -> usize {
        // Solved array is [1, 2, ..., n²-1, 0], so index is value-1,
        // except 0 which sits at the last position.
        if value == 0 {
            rank.pow(2) - 1
        } else {
            (value - 1) as usize
        }
    }

    pub fn solved_idx(&self, value: u8) -> usize {
        Self::solved_idx_for(usize::from(self), value)
    }

    pub fn is_even(&self) -> bool {
        (*self as u8).is_multiple_of(2)
    }
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

impl From<&Rank> for usize {
    fn from(rank: &Rank) -> usize {
        *rank as usize
    }
}

impl From<Rank> for i32 {
    fn from(rank: Rank) -> i32 {
        rank as i32
    }
}
