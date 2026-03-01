struct Puzzle {
    state: PuzzleState,
    previous_states: Vec<PuzzleState>,
}

struct PuzzleState {
    matrix: Vec<Vec<u8>>,
    cost_so_far: u16,
    move_counter: u16,
}