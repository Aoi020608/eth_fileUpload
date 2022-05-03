/*
#43

Given an array of positive integers nums and a positive integer target,
return the minimal length of a contiguous subarray [numsl, numsl+1, ..., numsr-1, numsr] of
which the sum is greater than or equal to target. If there is no such subarray, return 0 instead.

Input: target = 7, nums = [2,3,1,2,4,3]
Output: 2
Explanation: The subarray [4,3] has the minimal length under the problem constraint.


*/

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut queue = VecDeque::new();
        let mut left: usize = 0;
        let mut ret = 0;
        let mut sum = 0;
        let mut total_sum = 0;

        for r in 0..nums.len() {
            sum += nums[r];
            total_sum += nums[r];

            queue.push_back(nums[r]);

            while sum > target {
                let pop_val = queue.pop_front().unwrap();
                sum -= pop_val;
            }
        }

        println!("Total Sum: {:?}", total_sum);

        if total_sum < target {
            return 0;
        } else {
            ret = queue.len();
            if sum < target {
                return ret as i32 + 1;
            }
            ret as i32
        }
    }

    pub fn min_sub_array_len_1(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut sum = 0;
        let mut ans = nums.len();

        for i in 0..nums.len() {
            sum +=  nums[i];

            while sum - nums[left] >= target {
                sum -= nums[left];
                left += 1;
            }

            if sum >= target {
                ans = std::cmp::min(ans, i - left + 1);
            }
        }

        if sum < target {
            return 0;
        } else {
            return ans as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mun_sub_array_len() {
        let nums = vec![2, 3, 1, 2, 4, 3];
        let target = 7;
        let result = Solution::min_sub_array_len_1(target, nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_mun_sub_array_len_1() {
        let nums = vec![1, 4, 4];
        let target = 4;
        let result = Solution::min_sub_array_len_1(target, nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_mun_sub_array_len_2() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let target = 11;
        let result = Solution::min_sub_array_len_1(target, nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mun_sub_array_len_3() {
        let nums = vec![1, 2, 3, 4, 5];
        let target = 11;
        let result = Solution::min_sub_array_len_1(target, nums);
        assert_eq!(result, 3);
    }
}
