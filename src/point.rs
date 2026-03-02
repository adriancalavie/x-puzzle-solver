use num::Signed;

pub struct Point<T>
where
    T: Signed,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Signed,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Point<T>
where
    T: Signed,
{
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> Into<(T, T)> for Point<T>
where
    T: Signed,
{
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}
