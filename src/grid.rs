use std::fmt::Display;

use crate::Position;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T>
where
    T: Clone + Default + PartialEq + Eq + Display,
{
    data: Vec<T>,
    rank: usize,
}

impl<T> Grid<T>
where
    T: Clone + Default + PartialEq + Eq + Display,
{
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

    pub fn swap_values(&mut self, a: &Position, b: &Position) {
        let idx_a = self.index(a);
        let idx_b = self.index(b);
        self.data.swap(idx_a, idx_b); // stdlib, safe, no borrow issues
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

impl<T> Display for Grid<T>
where
    T: Clone + Default + PartialEq + Eq + Display,
{
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

impl<T> From<Vec<Vec<T>>> for Grid<T>
where
    T: Clone + Default + PartialEq + Eq + Display,
{
    fn from(matrix: Vec<Vec<T>>) -> Self {
        let rank = matrix.len();

        Self {
            data: matrix.into_iter().flatten().collect(),
            rank,
        }
    }
}

impl<T> From<Grid<T>> for Vec<Vec<T>>
where
    T: Clone + Default + PartialEq + Eq + Display,
{
    fn from(grid: Grid<T>) -> Self {
        grid.as_matrix()
    }
}

impl<T> PartialEq<[T]> for Grid<T>
where
    T: Clone + Default + PartialEq + Eq + Display,
{
    fn eq(&self, other: &[T]) -> bool {
        self.data == other
    }
}
