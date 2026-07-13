/*
You are given a 2D integer array occupiedIntervals, where occupiedIntervals[i] = [starti, endi]
represents a time interval during which you are occupied.
Each interval starts at starti and ends at endi, inclusive.
These intervals may overlap.

You are also given two integers freeStart and freeEnd, which define a
free time interval from freeStart to freeEnd, inclusive.

Your task is to merge all occupied intervals that overlap or touch,
then remove all integer points in the free interval from the merged occupied intervals.

Two intervals touch if the second interval starts immediately after the
first one ends. For example, [1, 1] and [2, 2] touch and should be merged into [1, 2].

Return the remaining occupied intervals in sorted order.
The returned intervals must be non-overlapping and must contain the
minimum number of intervals possible. If there are no remaining occupied points,
return an empty list.

Example 1:
    Input: occupiedIntervals = [[2,6],[4,8],[10,10],[10,12],[14,16]], freeStart = 7, freeEnd = 11
    Output: [[2,6],[12,12],[14,16]]
    Explanation:
        After merging, the occupied intervals are [2, 8], [10, 12], and [14, 16].
        Excluding the free interval [7, 11] results in [2, 6], [12, 12], and [14, 16].

Example 2:
    Input: occupiedIntervals = [[1,5],[2,3]], freeStart = 3, freeEnd = 8
    Output: [[1,2]]
    Explanation:
        After merging, the occupied interval is [1, 5].
        Excluding the free interval [3, 8] results in [1, 2].

Constraints:
    1 <= occupiedIntervals.length <= 5 * 104
    occupiedIntervals[i].length == 2
    1 <= starti <= endi <= 109
    1 <= freeStart <= freeEnd <= 109
*/

impl Solution {
    pub fn filter_occupied_intervals(
        mut occupied_intervals: Vec<Vec<i32>>,
        free_start: i32,
        free_end: i32,
    ) -> Vec<Vec<i32>> {
        if occupied_intervals.is_empty() {
            return vec![];
        }

        occupied_intervals.sort_by_key(|v| v[0]);

        let mut merged: Vec<(i64, i64)> = Vec::new();

        for interval in occupied_intervals {
            let s = interval[0] as i64;
            let e = interval[1] as i64;

            if let Some(prev) = merged.last_mut() {
                if prev.1 + 1 < s {
                    merged.push((s, e));
                } else {
                    prev.1 = prev.1.max(e);
                }
            } else {
                merged.push((s, e));
            }
        }

        let free_start = free_start as i64;
        let free_end = free_end as i64;
        let mut ans = Vec::new();

        for (s, e) in merged {
            if e < free_start || s > free_end {
                ans.push(vec![s as i32, e as i32]);
            } else {
                if s < free_start {
                    ans.push(vec![s as i32, (free_start - 1) as i32]);
                }

                if e > free_end {
                    ans.push(vec![(free_end + 1) as i32, e as i32]);
                }
            }
        }

        ans
    }
}
