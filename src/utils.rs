use std::{
    collections::HashMap,
    sync::{LazyLock, OnceLock},
};

use crate::Rank;

/// Overkill for this use-case, but it underlines how one could lazy load
/// expensive computations once per run.
/// In this case, if for example Rank::Five is never accessed, it's value is None
pub static SOLVED_CACHE: LazyLock<HashMap<Rank, OnceLock<Vec<u8>>>> = LazyLock::new(|| {
    [Rank::Two, Rank::Three, Rank::Four, Rank::Five]
        .into_iter()
        .map(|r| (r, OnceLock::new()))
        .collect()
});
