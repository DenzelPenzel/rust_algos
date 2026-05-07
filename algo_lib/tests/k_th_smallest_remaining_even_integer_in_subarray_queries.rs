/*
You are given an integer array nums where nums is strictly increasing.

You are also given a 2D integer array queries, 
where queries[i] = [li, ri, ki].

For each query [li, ri, ki]:

Consider the subarray nums[li..ri]
From the infinite sequence of all 
positive even integers: 2, 4, 6, 8, 10, 12, 14, ...

Remove all elements that appear in the subarray nums[li..ri].

Find the kith smallest integer remaining in the 
sequence after the removals.

Return an integer array ans, where ans[i] is the result for the ith query.

Example 1:
    Input: nums = [1,4,7], queries = [[0,2,1],[1,1,2],[0,0,3]]
    Output: [2,6,6]
    Explanation:
        i	queries[i]	nums[li..ri]	Removed
        Evens	Remaining
        Evens	ki	ans[i]
        0	[0, 2, 1]	[1, 4, 7]	[4]	2, 6, 8, ...	1	2
        1	[1, 1, 2]	[4]	[4]	2, 6, 8, ...	2	6
        2	[0, 0, 3]	[1]	[]	2, 4, 6, ...	3	6
        Thus, ans = [2, 6, 6].

Example 2:
    Input: nums = [2,5,8], queries = [[0,1,2],[1,2,1],[0,2,4]]
    Output: [6,2,12]
    Explanation:
        i	queries[i]	nums[li..ri]	Removed
        Evens	Remaining
        Evens	ki	ans[i]
        0	[0, 1, 2]	[2, 5]	[2]	4, 6, 8, ...	2	6
        1	[1, 2, 1]	[5, 8]	[8]	2, 4, 6, ...	1	2
        2	[0, 2, 4]	[2, 5, 8]	[2, 8]	4, 6, 10, 12, ...	4	12
        Thus, ans = [6, 2, 12].

Example 3:
    Input: nums = [3,6], queries = [[0,1,1],[1,1,3]]
    Output: [2,8]
    Explanation:
        i	queries[i]	nums[li..ri]	Removed
        Evens	Remaining
        Evens	ki	ans[i]
        0	[0, 1, 1]	[3, 6]	[6]	2, 4, 8, ...	1	2
        1	[1, 1, 3]	[6]	[6]	2, 4, 8, ...	3	8
        Thus, ans = [2, 8].

Constraints:
    1 <= nums.length <= 105
    1 <= nums[i] <= 109
    nums is strictly increasing
    1 <= queries.length <= 105
    queries[i] = [li, ri, ki]
    0 <= li <= ri < nums.length
    1 <= ki <= 109​​​​​​​

*/



impl Solution {
    pub fn kth_remaining_integer(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + if nums[i] % 2 == 0 { 1 } else { 0 };
        }

        let mut res = Vec::with_capacity(queries.len());
        
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as i64;

            let mut low = k;
            let mut high = k + (r - l + 1) as i64;
            let mut best_m = high;

            while low <= high {
                let mid = low + ((high - low) >> 1);
                let x = mid * 2;
                
                // Find how many elements in the subarray are <= x
                let cnt = nums[l..=r].partition_point(|&val| (val as i64) <= x as i64);

                // Use the prefix array to find how many of those elements are even
                let evens_count = prefix[l + cnt] - prefix[l];

                // Calculate the remaining available even numbers <= x
                let valid = mid - evens_count as i64;

                if valid >= k {
                    best_m = mid;
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }

            res.push((best_m * 2) as i32);
        }

        res
    }
}

