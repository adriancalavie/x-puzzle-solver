use x_puzzle_solver::{Puzzle, Rank};

#[test]
fn create_puzzle_rank_3() {
    let puzzle = Puzzle::from_str("0 1 2\n3 4 5\n6 7 8\n").unwrap();
    let rendered = format!("{puzzle}");
    let expected = "Matrix:\n0 1 2 \n3 4 5 \n6 7 8 \nRank: 3\nCost so far: 0\nEmpty tile: [0,0]\nMove counter: 0";
    assert_eq!(rendered, expected);
    assert_eq!(puzzle.get_rank(), Rank::Three);
}

#[test]
fn create_puzzle_rank_4() {
    let puzzle = Puzzle::from_str("0 1 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15").unwrap();
    let rendered = format!("{puzzle}");
    let expected = "Matrix:\n0 1 2 3 \n4 5 6 7 \n8 9 10 11 \n12 13 14 15 \nRank: 4\nCost so far: 0\nEmpty tile: [0,0]\nMove counter: 0";
    assert_eq!(rendered, expected);
    assert_eq!(puzzle.get_rank(), Rank::Four);
}

#[test]
fn create_puzzle_rank_5() {
    let puzzle =
        Puzzle::from_str("0 1 2 3 4\n5 6 7 8 9\n10 11 12 13 14\n15 16 17 18 19\n20 21 22 23 24")
            .unwrap();
    let rendered = format!("{puzzle}");
    let expected = "Matrix:\n0 1 2 3 4 \n5 6 7 8 9 \n10 11 12 13 14 \n15 16 17 18 19 \n20 21 22 23 24 \nRank: 5\nCost so far: 0\nEmpty tile: [0,0]\nMove counter: 0";
    assert_eq!(rendered, expected);
    assert_eq!(puzzle.get_rank(), Rank::Five);
}
