use std::fmt::Display;

use crate::Position;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    data: Vec<u8>,
    pub(super) rank: usize,
}

impl Grid {
    pub fn new(cells: Vec<u8>, rank: usize) -> Self {
        Self { data: cells, rank }
    }

    pub fn as_matrix(&self) -> Vec<Vec<u8>> {
        let mut matrix = vec![vec![0; self.rank]; self.rank];

        for (idx, val) in self.data.iter().enumerate() {
            let pos = self.index_to_pos(idx);
            matrix[pos.y][pos.x] = *val;
        }
        matrix
    }

    pub fn as_rows(&self) -> Vec<Vec<u8>> {
        self.as_matrix()
    }

    pub fn as_cols(&self) -> Vec<Vec<u8>> {
        let mut cols = vec![vec![0; self.rank]; self.rank];

        for (idx, val) in self.data.iter().enumerate() {
            let pos = self.index_to_pos(idx);
            cols[pos.x][pos.y] = *val;
        }
        cols
    }

    pub fn at(&self, pos: Position) -> u8 {
        self.data[self.index(&pos)]
    }

    pub fn swap_values(&self, a: &Position, b: &Position) -> Self {
        let idx_a = self.index(a);
        let idx_b = self.index(b);

        let mut new_data: Vec<u8> = self.data.clone();
        new_data.swap(idx_a, idx_b);
        Self::new(new_data, self.rank)
    }

    pub fn count_inversions(&self) -> usize {
        let mut inversions = 0;
        for (i, val) in self.data.iter().enumerate() {
            for j in i + 1..self.data.len() {
                if *val == 0 {
                    continue;
                }
                if self.data[j] == 0 {
                    continue;
                }
                if self.data[j] < *val {
                    inversions += 1;
                }
            }
        }
        inversions
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn index(&self, pos: &Position) -> usize {
        pos.y * self.rank + pos.x
    }

    pub fn index_to_pos(&self, idx: usize) -> Position {
        Position {
            x: idx % self.rank,
            y: idx / self.rank,
        }
    }
}

impl Display for Grid {
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

impl From<Vec<Vec<u8>>> for Grid {
    fn from(matrix: Vec<Vec<u8>>) -> Self {
        let rank = matrix.len();

        Self {
            data: matrix.into_iter().flatten().collect(),
            rank,
        }
    }
}

impl From<Grid> for Vec<Vec<u8>> {
    fn from(grid: Grid) -> Self {
        grid.as_matrix()
    }
}

impl PartialEq<[u8]> for Grid {
    fn eq(&self, other: &[u8]) -> bool {
        self.data == other
    }
}
