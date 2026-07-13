/*
You are given an integer array nums and two integers k and mul.

Select exactly k elements from nums. Process these elements one by one in any order you choose.

For each selected element, independently choose one of the following:

Add the element's value to the total sum, or
Multiply the element by the current value of mul and add the result to the total sum.
After processing each selected element, mul decreases by 1, regardless of which option was chosen.
The current value of mul may become 0 or negative.

Return an integer denoting the maximum possible total sum.

Example 1:
    Input: nums = [6,1,2,9], k = 3, mul = 2
    Output: 26
    Explanation:
        One optimal way:

        One optimal selection is nums[3] = 9, nums[0] = 6, and nums[2] = 2.
        Process nums[3] = 9 first: choose multiplication, so it contributes 9 * 2 = 18. Now, mul becomes 1.
        Process nums[0] = 6 next: choose multiplication, so it contributes 6 * 1 = 6. Now, mul becomes 0.
        Process nums[2] = 2 last: choose addition, so it contributes 2.
        The total sum is 18 + 6 + 2 = 26.

Example 2:
    Input: nums = [3,7,5,2], k = 2, mul = 4
    Output: 43
    Explanation:
        One optimal way:

        One optimal selection is nums[1] = 7 and nums[2] = 5.
        Process nums[1] = 7 first: choose multiplication, so it contributes 7 * 4 = 28. Now, mul becomes 3.
        Process nums[2] = 5 next: choose multiplication, so it contributes 5 * 3 = 15.
        The total sum is 28 + 15 = 43.

Example 3:
    Input: nums = [4,4], k = 1, mul = 1
    Output: 4
        Explanation:
            One optimal way:
            One optimal selection is nums[0] = 4.
            Process nums[0] = 4: choose multiplication, so it contributes 4 * 1 = 4.
            The total sum is 4.

Constraints:
    1 <= nums.length <= 105
    1 <= nums[i] <= 105
    1 <= k <= nums.length
    1 <= mul <= 105
*/

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32, mul: i32) -> i64 {
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        let mut res = 0i64;
        let mut zeros = 0;

        for num in nums {
            if num > 0 {
                pos.push(num as i64);
            } else if num < 0 {
                neg.push(num as i64);
            } else {
                zeros += 1;
            }
        }

        pos.sort_unstable_by(|a, b| b.cmp(a));
        neg.sort_unstable();

        let mut i = 0;
        let mut j = 0;

        for s in 0..k as usize {
            let cur_mul = mul as i64 - s as i64;
            let mut best = i64::MIN;
            let mut take = 0;

            if i < pos.len() {
                let x = pos[i];
                let val = x.max(x * cur_mul);

                if val > best {
                    best = val;
                    take = 1;
                }
            }

            if j < neg.len() {
                let x = neg[i];
                let val = x.max(x * cur_mul);

                if val > best {
                    best = val;
                    take = 2;
                }
            }

            // just take 0
            if zeros > 0 && 0 > best {
                best = 0;
                take = 3;
            }

            if take == 1 {
                i += 1;
            } else if take == 2 {
                j += 1
            } else {
                zeros -= 1;
            }

            res += best;
        }

        res as i64
    }
}
