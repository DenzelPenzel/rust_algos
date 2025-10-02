/// Algorithm library
/// 
/// This library contains various algorithm implementations

pub mod collections;
pub mod graph;
pub mod helpers;
pub mod numbers;
mod problems;

/// Adds two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
    }
}
