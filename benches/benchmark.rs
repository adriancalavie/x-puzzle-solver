use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use x_puzzle_solver::Puzzle;
use x_puzzle_solver::Rank;

fn benchmark_puzzle_from_string(c: &mut Criterion) {
    let inputs = vec![
        ("2x2", "0 1\n2 3"),
        ("3x3", "0 1 2\n3 4 5\n6 7 8"),
        ("4x4", "0 1 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15"),
        (
            "5x5",
            "0 1 2 3 4\n5 6 7 8 9\n10 11 12 13 14\n15 16 17 18 19\n20 21 22 23 24",
        ),
    ];

    let mut group = c.benchmark_group("Puzzle from string creation");
    for (label, matrix) in inputs {
        group.bench_with_input(label, &matrix, |b, m| {
            b.iter(|| black_box(m).parse::<Puzzle>().unwrap())
        });
    }

    group.finish();
}

fn benchmark_rank_functionality(c: &mut Criterion) {
    let inputs = vec![
        ("rank 2", Rank::Two),
        ("rank 3", Rank::Three),
        ("rank 4", Rank::Four),
        ("rank 5", Rank::Five),
    ];

    let mut group = c.benchmark_group("Solved arrays per rank");
    for (label, rank) in inputs {
        group.bench_with_input(label, &rank, |b, r| b.iter(|| black_box(r.get_solved())));
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_puzzle_from_string,
    benchmark_rank_functionality
);
criterion_main!(benches);
