use num::{Signed, abs};

pub fn compute_manhattan<T>(x1: T, y1: T, x2: T, y2: T) -> T
where
    T: Signed,
{
    abs(x1 - x2) + abs(y1 - y2)
}

##[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        
    }
}