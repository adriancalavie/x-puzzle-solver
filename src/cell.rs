use crate::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell<T>
where
    T: Clone,
{
    pub value: T,
    pub position: Point,
}

impl<T> Cell<T>
where
    T: Clone,
{
    pub fn new(value: T, position: Point) -> Self {
        Self { value, position }
    }

    pub fn switch_value_mut(&mut self, other: &mut Cell<T>) {
        std::mem::swap(&mut self.value, &mut other.value);
    }

    pub fn switch_value(lhs: &Cell<T>, rhs: &Cell<T>) -> (Cell<T>, Cell<T>) {
        let mut cell_1 = lhs.clone();
        let mut cell_2 = rhs.clone();
        cell_1.switch_value_mut(&mut cell_2);
        (cell_1, cell_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_switch_mut() {
        let mut cell_1 = Cell::new("value of cell one", Point::new(0, 0));
        let mut cell_2 = Cell::new("value of cell two", Point::new(1, 0));
        cell_1.switch_value_mut(&mut cell_2);
        assert_eq!(cell_1.value, "value of cell two");
        assert_eq!(cell_2.value, "value of cell one");
        assert_ne!(cell_1, cell_2);
        assert_ne!(&cell_1, &cell_2);
    }

    #[test]
    fn cell_switch() {
        let cell_1 = Cell::new("value of cell one", Point::new(0, 0));
        let cell_2 = Cell::new("value of cell two", Point::new(1, 0));
        let (cell_1, cell_2) = Cell::switch_value(&cell_1, &cell_2);
        assert_eq!(cell_1.value, "value of cell two");
        assert_eq!(cell_2.value, "value of cell one");
        assert_ne!(cell_1, cell_2);
        assert_ne!(&cell_1, &cell_2);
    }
}
