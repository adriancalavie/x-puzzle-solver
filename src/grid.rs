use std::fmt::Display;

use crate::{Cell, Point};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    cells: Vec<Cell<i32>>,
}

impl Grid {
    pub fn new(cells: Vec<Cell<i32>>) -> Self {
        Self { cells }
    }

    pub fn as_matrix(&self) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; self.cells.len().isqrt()]; self.cells.len().isqrt()];
        for (_, cell) in self.cells.iter().enumerate() {
            matrix[cell.position.y as usize][cell.position.x as usize] = cell.value;
        }
        matrix
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

impl From<Vec<Vec<i32>>> for Grid {
    fn from(matrix: Vec<Vec<i32>>) -> Self {
        let cells = matrix
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter().enumerate().map(move |(x, value)| Cell {
                    position: Point::new(x as i32, y as i32),
                    value,
                })
            })
            .collect();
        Self { cells }
    }
}

impl From<Grid> for Vec<Vec<i32>> {
    fn from(grid: Grid) -> Self {
        grid.as_matrix()
    }
}
