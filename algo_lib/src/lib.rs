/// Algorithm library
///
/// This library contains various algorithm implementations
pub mod collections;
pub mod graph;
pub mod helpers;
pub mod io;
pub mod numbers;
pub mod string;

/// Adds two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn remaining_occupied_intervals(
    mut occupied_intervals: Vec<Vec<i32>>,
    free_start: i32,
    free_end: i32,
) -> Vec<Vec<i32>> {
    occupied_intervals.sort_unstable_by_key(|interval| (interval[0], interval[1]));

    let mut merged = Vec::<[i32; 2]>::new();
    for interval in occupied_intervals {
        let start = interval[0];
        let end = interval[1];

        if let Some(last) = merged.last_mut() {
            if start <= last[1].saturating_add(1) {
                last[1] = last[1].max(end);
                continue;
            }
        }

        merged.push([start, end]);
    }

    let mut result = Vec::new();
    for [start, end] in merged {
        if end < free_start || start > free_end {
            result.push(vec![start, end]);
            continue;
        }

        if start < free_start {
            result.push(vec![start, free_start - 1]);
        }

        if end > free_end {
            result.push(vec![free_end + 1, end]);
        }
    }

    result
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

    #[test]
    fn remaining_occupied_intervals_merges_touching_then_removes_free_range() {
        let occupied = vec![vec![5, 8], vec![1, 2], vec![3, 4], vec![10, 12]];

        let result = remaining_occupied_intervals(occupied, 6, 10);

        assert_eq!(result, vec![vec![1, 5], vec![11, 12]]);
    }

    #[test]
    fn remaining_occupied_intervals_splits_interval_around_free_range() {
        let occupied = vec![vec![1, 10]];

        let result = remaining_occupied_intervals(occupied, 4, 6);

        assert_eq!(result, vec![vec![1, 3], vec![7, 10]]);
    }

    #[test]
    fn remaining_occupied_intervals_returns_empty_when_free_range_covers_everything() {
        let occupied = vec![vec![1, 1], vec![2, 3]];

        let result = remaining_occupied_intervals(occupied, 1, 3);

        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn remaining_occupied_intervals_keeps_non_overlapping_intervals_sorted() {
        let occupied = vec![vec![4, 4], vec![1, 2]];

        let result = remaining_occupied_intervals(occupied, 10, 12);

        assert_eq!(result, vec![vec![1, 2], vec![4, 4]]);
    }
}
