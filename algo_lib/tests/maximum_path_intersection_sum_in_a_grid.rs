/*
You are given an m x n integer matrix grid.

Two players move across the grid:

Player 1 starts at the top-left cell (0, 0) and can move only right or down. 
Their destination is the bottom-right cell (m - 1, n - 1).
Player 2 starts at the bottom-left cell (m - 1, 0) and can move only right or up. 
Their destination is the top-right cell (0, n - 1).
Each player must choose a valid path from their respective starting cell to their destination.

A cell is called shared if it belongs to both chosen paths.

Return an integer denoting the maximum possible sum of values of all shared cells.

Example 1:    ​​​​​​​​​​​​​​​​​​​​​
    Input: grid = [[1,2,0,-3],[1,-2,1,0],[-4,2,-1,3],[3,-3,3,-2],[-1,-5,0,1]]
    Output: 4
    Explanation:
        The diagram shows one optimal choice of paths.
        Player 1 follows the red/purple path from the top-left cell to the bottom-right cell:
        (0, 0) → (1, 0) → (2, 0) → (2, 1) → (2, 2) → (2, 3) → (3, 3) → (4, 3)
        Player 2 follows the blue/purple path from the bottom-left cell to the top-right cell:
        (4, 0) → (4, 1) → (3, 1) → (2, 1) → (2, 2) → (2, 3) → (1, 3) → (0, 3)
        The shared cells are (2, 1), (2, 2), and (2, 3).
        The sum is 2 + (-1) + 3 = 4, which is the maximum possible sum.

Example 2:
    Input: grid = [[4,-2,-3],[-1,-3,-1],[-4,2,-1]]
    Output: 3
    Explanation:
        One optimal pair of paths is shown in the diagram.

        Player 1 follows the red/purple path:
        (0, 0) → (1, 0) → (1, 1) → (1, 2) → (2, 2)
        Player 2 follows the blue/purple path:
        (2, 0) → (1, 0) → (0, 0) → (0, 1) → (0, 2)
        The shared cells are (0, 0) and (1, 0).
        The sum is 4 + (-1) = 3, which is the maximum possible.
    
Constraints:
    m == grid.length
    n == grid[i].length
    2 <= m, n <= 1000
    4 <= m * n <= 5 * 105
    -100 <= grid[i][j] <= 100
*/

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut res = i64::MIN;

        for i in 0..n {
            let mut best_end = grid[i][0] as i64;

            for j in 1..m {
                let x = grid[i][j] as i64;
                res = res.max(best_end + x);
                best_end = x.max(best_end + x);
            }
        }

        for j in 0..m {
            let mut best_end = grid[0][j] as i64;

            for i in 1..n {
                let x = grid[i][j] as i64;
                res = res.max(best_end + x);
                best_end = x.max(best_end + x);
            }
        }

        for i in 1..n - 1 {
            for j in 1..m - 1 {
                res = res.max(grid[i][j] as i64);
            }
        }

        res as i32
    }
}
