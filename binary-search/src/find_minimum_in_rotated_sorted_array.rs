/*
Suppose an array of length n sorted in ascending order is rotated between 1 and n times.
For example, the array nums = [0,1,2,4,5,6,7] might become:

[4,5,6,7,0,1,2] if it was rotated 4 times.
[0,1,2,4,5,6,7] if it was rotated 7 times.
Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].

Given the sorted rotated array nums of unique elements, return the minimum element of this array.

You must write an algorithm that runs in O(log n) time.

Input: nums = [3,4,5,1,2]
Output: 1
Explanation: The original array was [1,2,3,4,5] rotated 3 times.

*/

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut left_index = 0;
        let mut right_index = nums.len() - 1;

        while left_index <= right_index {
            if nums[left_index] < nums[right_index] {
                res = std::cmp::min(res, nums[left_index]);
                break;
            }

            let mut mid = (left_index + right_index) / 2;
            res = std::cmp::min(res, nums[mid]);

            if nums[mid] >= nums[left_index] {
                left_index = mid + 1;
            } else {
                right_index = mid - 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_min() {
        let nums = vec![3, 4, 5, 1, 2];
        let result = Solution::find_min(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_min_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let result = Solution::find_min(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_find_min_2() {
        let nums = vec![11, 13, 15, 17];
        let result = Solution::find_min(nums);
        assert_eq!(result, 11);
    }
}
