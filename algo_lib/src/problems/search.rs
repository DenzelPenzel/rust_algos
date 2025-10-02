pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len();

        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            if nums[mid] < target {
                lo = mid + 1
            } else {
                hi = mid
            }
        }

        if lo >= 0 && lo < nums.len() && nums[lo] == target {
            return lo as i32;
        }

        -1
    }
}