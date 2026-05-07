/*
You are given two integer arrays nums1 and nums0, each of size n.

nums1[i] represents the number of '1's in the ith segment.
nums0[i] represents the number of '0's in the ith segment.
For each index i, construct a binary segment consisting of:

nums1[i] occurrences of '1' followed by
nums0[i] occurrences of '0'.
You may rearrange the order of these segments in any way.
 After rearranging, concatenate all segments to form a single binary string.

Return the maximum possible integer value of 
the concatenated binary string.

Since the result can be very large, return the answer modulo 109 + 7.

Example 1:
    Input: nums1 = [1,2], nums0 = [1,0]
    Output: 14
    Explanation:
        At index 0, nums1[0] = 1 and nums0[0] = 1, so the segment formed is "10".
        At index 1, nums1[1] = 2 and nums0[1] = 0, so the segment formed is "11".
        Reordering the segments as "11" followed by "10" produces the binary string "1110".
        The binary number "1110" has value 14 which is the maximum possible value.

Example 2:
    Input: nums1 = [3,1], nums0 = [0,3]
    Output: 120
    Explanation:
        At index 0, nums1[0] = 3 and nums0[0] = 0, so the segment formed is "111".
        At index 1, nums1[1] = 1 and nums0[1] = 3, so the segment formed is "1000".
        Reordering the segments as "111" followed by "1000" produces the binary string "1111000".
        The binary number "1111000" has value 120 which is the maximum possible value.
    
Constraints:
    1 <= n == nums1.length == nums0.length <= 105
    0 <= nums1[i], nums0[i] <= 104
    nums1[i] + nums0[i] > 0
    The total sum of all elements in nums1 and nums0 does not exceed 2 * 105.

*/


use std::cmp::Ordering;


impl Solution {
    pub fn max_value(nums1: Vec<i32>, nums0: Vec<i32>) -> i32 {
        fn pow(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
            let mut res = 1;
            base %= modulo;
            while exp > 0 {
                if exp % 2 == 1 {
                    res = (res * base) % modulo;
                }
                base = (base * base) % modulo;
                exp /= 2;
            }
            res
        }

        fn rle(ones1: u64, zeros1: u64, ones2: u64, zeros2: u64) -> Vec<(u8, u64)> {
            let mut res: Vec<(u8, u64)> = Vec::with_capacity(4);
            
            for &(val, cnt) in &[(1, ones1), (0, zeros1), (1, ones2), (0, zeros2)] {
                if cnt > 0 {
                    if let Some(lst) = res.last_mut() {
                        if lst.0 == val {
                            lst.1 += cnt;
                            continue;
                        }
                    }
                    res.push((val, cnt));
                }
            }
            
            res
        }
        
        let modulo = 1_000_000_007_u64;
        let n = nums1.len();
        let mut seg: Vec<(u64, u64)> = (0..n)
            .map(|i| (nums1[i] as u64, nums0[i] as u64))
            .collect();

        seg.sort_unstable_by(|a, b| {
            // [val, cnt]
            let mut x = rle(a.0, a.1, b.0, b.1);
            let mut y = rle(b.0, b.1, a.0, a.1);

            let mut i = 0;
            let mut j = 0;

            while i < x.len() && j < y.len() {
                if x[i].0 != y[j].0 {
                    return y[j].0.cmp(&x[i].0);
                }

                let min_len = x[i].1.min(y[j].1);
                x[i].1 -= min_len;
                y[j].1 -= min_len;

                if x[i].1 == 0 {
                    i += 1;
                }
                if y[j].1 == 0 {
                    j += 1;
                }
            }
            Ordering::Equal
        });

        let mut res = 0;

        for (ones, zeros) in seg {
            let total_len = ones + zeros;

            res = (res * pow(2, total_len, modulo)) % modulo;

            let ones_val = (pow(2, ones,  modulo) + modulo - 1) % modulo;
            let seg_val = (ones_val * pow(2, zeros, modulo)) % modulo;

            res = (res + seg_val) % modulo;
        }

        res as i32
    }
}

