use std::fmt::Display;

use crate::Position;

pub trait GridDataType: Clone + Default + Ord + Display {}
impl<T: Clone + Default + Ord + Display> GridDataType for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T: GridDataType> {
    data: Vec<T>,
    rank: usize,
}

impl<T: GridDataType> Grid<T> {
    pub fn new(cells: Vec<T>, rank: usize) -> Self {
        Self { data: cells, rank }
    }

    pub fn as_matrix(&self) -> Vec<Vec<T>> {
        let mut matrix = vec![vec![T::default(); self.rank]; self.rank];
        for (idx, val) in self.data.iter().enumerate() {
            let pos = self.index_to_pos(idx);
            matrix[pos.y][pos.x] = val.clone();
        }
        matrix
    }

    pub fn at(&self, pos: Position) -> T {
        self.data[self.index(&pos)].clone()
    }

    pub fn swap_values(&self, a: &Position, b: &Position) -> Self {
        let idx_a = self.index(a);
        let idx_b = self.index(b);

        let mut new_data: Vec<T> = self.data.clone();
        new_data.swap(idx_a, idx_b);
        Self::new(new_data, self.rank)
    }

    pub fn count_inversions(&self) -> usize {
        let mut inversions = 0;
        for (i, val) in self.data.iter().enumerate() {
            for j in i + 1..self.data.len() {
                if *val == T::default() {
                    // not zero
                    continue;
                }
                if self.data[j] == T::default() {
                    // not zero
                    continue;
                }
                if self.data[j] < *val {
                    inversions += 1;
                }
            }
        }
        inversions
    }

    fn index(&self, pos: &Position) -> usize {
        pos.y * self.rank + pos.x
    }

    fn index_to_pos(&self, idx: usize) -> Position {
        Position {
            x: idx % self.rank,
            y: idx / self.rank,
        }
    }
}

impl<T: GridDataType> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.as_matrix() {
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: GridDataType> From<Vec<Vec<T>>> for Grid<T> {
    fn from(matrix: Vec<Vec<T>>) -> Self {
        let rank = matrix.len();

        Self {
            data: matrix.into_iter().flatten().collect(),
            rank,
        }
    }
}

impl<T: GridDataType> From<Grid<T>> for Vec<Vec<T>> {
    fn from(grid: Grid<T>) -> Self {
        grid.as_matrix()
    }
}

impl<T: GridDataType> PartialEq<[T]> for Grid<T> {
    fn eq(&self, other: &[T]) -> bool {
        self.data == other
    }
}
