use crate::utils::SOLVED_CACHE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

impl Rank {
    fn compute_solved(&self) -> Vec<i32> {
        let size = usize::from(self).pow(2);
        (1..=size)
            .map(|i| if i == size { 0 } else { i as i32 })
            .collect()
    }

    /// lazy inits, cached reads
    /// See [SOLVED_CACHE](crate::utils::SOLVED_CACHE) for details
    pub fn get_solved(&self) -> &'static [i32] {
        SOLVED_CACHE[self].get_or_init(|| self.compute_solved())
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
