/*

You are given two integer arrays costs and capacity, both of length n, 
where costs[i] represents the purchase cost of the ith machine and capacity[i] represents its performance capacity.

You are also given an integer budget.

You may select at most two distinct machines such that the total cost of the selected machines is strictly less than budget.

Return the maximum achievable total capacity of the selected machines.

Example 1:
    Input: costs = [4,8,5,3], capacity = [1,5,2,7], budget = 8
    Output: 8
    Explanation:
        Choose two machines with costs[0] = 4 and costs[3] = 3.
        The total cost is 4 + 3 = 7, which is strictly less than budget = 8.
        The maximum total capacity is capacity[0] + capacity[3] = 1 + 7 = 8.

Example 2:
    Input: costs = [3,5,7,4], capacity = [2,4,3,6], budget = 7
    Output: 6
    Explanation:
        Choose one machine with costs[3] = 4.
        The total cost is 4, which is strictly less than budget = 7.
        The maximum total capacity is capacity[3] = 6.

Example 3:
    Input: costs = [2,2,2], capacity = [3,5,4], budget = 5
    Output: 9
    Explanation:
        Choose two machines with costs[1] = 2 and costs[2] = 2.
        The total cost is 2 + 2 = 4, which is strictly less than budget = 5.
        The maximum total capacity is capacity[1] + capacity[2] = 5 + 4 = 9.
    
Constraints:
    1 <= n == costs.length == capacity.length <= 105
    1 <= costs[i], capacity[i] <= 105
    1 <= budget <= 2 * 105

*/

use std::cmp;

struct Solution;

impl Solution {
    pub fn max_capacity(costs: Vec<i32>, capacity: Vec<i32>, budget: i32) -> i32 {
        let n = costs.len();
        let mut res = 0;
        let mut tmp = Vec::with_capacity(n);
        
        for i in 0..n {
            if costs[i] < budget {
                res = cmp::max(res, capacity[i])
            }
            tmp.push((costs[i], capacity[i]));
        }

        tmp.sort_unstable_by_key(|k| k.0);

        let mut max_capacity = vec![0; n];
        let mut current_max = 0;
        let mut sorted_costs = vec![0; n];
        for i in 0..n {
            current_max = cmp::max(current_max, tmp[i].1);
            max_capacity[i] = current_max;
            sorted_costs[i] = tmp[i].0;
        }

        for j in 0..n {
            if tmp[j].0 >= budget {
                break;
            }
            let rem = budget - tmp[j].0;
            let i =  sorted_costs.partition_point(|&x| x < rem);
            let mut i_idx = i as isize - 1;

            if i_idx >= j as isize {
                i_idx = j as isize - 1;
            }

            if i_idx >= 0 {
                res = cmp::max(res, tmp[j].1 + max_capacity[i_idx as usize]);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_capacity() {
        let costs = vec![4,8,5,3];
        let capacity = vec![1,5,2,7];
        let budget = 8;
        let cap = Solution::max_capacity(costs, capacity, budget);
        assert_eq!(cap, 8)
    }
}