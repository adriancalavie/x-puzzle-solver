use x_puzzle_solver::Puzzle;

#[test]
fn create_puzzle_rank_3() {
    let puzzle = Puzzle::from_str("0 1 2\n3 4 5\n6 7 8\n").unwrap();
    let rendered = format!("{puzzle}");
    let expected = "Matrix:\n0 1 2 \n3 4 5 \n6 7 8 \nRank: 3\nCost so far: 0\nMove counter: 0";
    assert_eq!(rendered, expected);
}
