use std::str::FromStr;

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

#[test]
fn solve_puzzle_rank_3() {
    let puzzle = Puzzle::from_str("0 1 2\n3 4 5\n6 7 8\n").unwrap();

    let result = puzzle.solve();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().move_counter, 22)
}

#[test]
fn solve_puzzle_rank_4() {
    let puzzle = Puzzle::from_str("5 1 3 11\n2 7 8 4\n9 6 0 12\n13 15 10 14").unwrap();

    let result = puzzle.solve();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().move_counter, 26)
}

#[test]
fn solve_puzzle_rank_4_hard() {
    let puzzle = Puzzle::from_str("2 7 8 15\n4 14 5 13\n6 12 11 10\n1 3 9 0").unwrap();

    let result = puzzle.solve();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().move_counter, 52)
}

// #[test]
// fn solve_puzzle_rank_5_hard() {
//     let puzzle =
//         Puzzle::from_str("21 2 4 6 15\n24 8 9 20 7\n18 22 14 1 16\n0 3 13 5 12\n11 17 10 23 19")
//             .unwrap();

//     let result = puzzle.solve();

//     assert!(result.is_ok());
//     assert_eq!(result.unwrap().move_counter, 52)
// }
