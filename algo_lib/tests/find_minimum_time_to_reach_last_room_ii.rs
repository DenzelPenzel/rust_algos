/*
There is a dungeon with n x m rooms arranged as a grid.

You are given a 2D array moveTime of size n x m, where moveTime[i][j] represents the minimum 
time in seconds when you can start moving to that room. You start from the room (0, 0) at time t = 0 
and can move to an adjacent room. Moving between adjacent rooms takes one second for one move and 
two seconds for the next, alternating between the two.

Return the minimum time to reach the room (n - 1, m - 1).

Two rooms are adjacent if they share a common wall, either horizontally or vertically.

Example 1:
    Input: moveTime = [[0,4],[4,4]]
    Output: 7
    Explanation:
        The minimum time required is 7 seconds.
        At time t == 4, move from room (0, 0) to room (1, 0) in one second.
        At time t == 5, move from room (1, 0) to room (1, 1) in two seconds.

Example 2:
    Input: moveTime = [[0,0,0,0],[0,0,0,0]]
    Output: 6
    Explanation:
        The minimum time required is 6 seconds.
        At time t == 0, move from room (0, 0) to room (1, 0) in one second.
        At time t == 1, move from room (1, 0) to room (1, 1) in two seconds.
        At time t == 3, move from room (1, 1) to room (1, 2) in one second.
        At time t == 4, move from room (1, 2) to room (1, 3) in two seconds.

Example 3:
    Input: moveTime = [[0,1],[1,2]]
    Output: 4

Constraints:
    2 <= n == moveTime.length <= 750
    2 <= m == moveTime[i].length <= 750
    0 <= moveTime[i][j] <= 109
*/

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();
        let mut dist = vec![vec![i32::MAX; m]; n];
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut pq = BinaryHeap::new();
        
        dist[0][0] = 0;
        pq.push((Reverse(0), 1, 0, 0));
        
        while let Some((Reverse(d), step, i, j)) = pq.pop() {
            if i == n - 1 && j == m - 1 {
                return d;
            }

            if d > dist[i][j] {
                continue;
            }

            for (dx, dy) in directions {
                let x = i as isize + dx;
                let y = j as isize + dy;
                
                if x >= 0 && x < n as isize && y >= 0 && y < m as isize {
                    let x = x as usize;
                    let y = y as usize;
                    let next_step = if step == 1 {
                        2
                    } else {
                        1
                    };
                    if d >= move_time[x][y] {
                        if dist[i][j] + step < dist[x][y] {
                            dist[x][y] = dist[i][j] + step;
                            pq.push((Reverse(dist[x][y]), next_step, x, y));
                        }  
                    } else {
                        if move_time[x][y] + step < dist[x][y] {
                            dist[x][y] = move_time[x][y] + step;
                            pq.push((Reverse(move_time[x][y] + step), next_step, x, y));    
                        }                        
                        
                    }
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_time_to_reach() {
        let move_time = vec![vec![0, 4], vec![4, 4]];
        assert_eq!(Solution::min_time_to_reach(move_time), 7);

        let move_time = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
        assert_eq!(Solution::min_time_to_reach(move_time), 6);
    }
}
