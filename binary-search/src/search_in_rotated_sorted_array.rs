/*
There is an integer array nums sorted in ascending order (with distinct values).

Prior to being passed to your function,
nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length)
such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed).
For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

Given the array nums after the possible rotation and an integer target,
return the index of target if it is in nums, or -1 if it is not in nums.

You must write an algorithm with O(log n) runtime complexity.

Input: nums = [4,5,6,7,0,1,2], target = 0
Output: 4

*/

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = -1;
        let mut left_index = 0;
        let mut right_index = nums.len() - 1;

        while left_index < right_index {
            let mut mid = (left_index + right_index) / 2;

            if nums[mid] == target {
                res = mid as i32;
                // return mid as i32;
                break;
            }

            if nums[left_index] == target {
                res = left_index as i32;
                break;
            }

            if nums[right_index] == target {
                res = right_index as i32;
                break;
            }

            if nums[left_index] <= target && target <= nums[mid] {
                right_index = mid - 1;
            } else {
                left_index = mid + 1;
            }
        }

        res
    }

    pub fn search_01(nums: Vec<i32>, target: i32) -> i32 {
        match nums.iter().position(|&x| x == target) {
            Some(index) => return index as i32,
            None => return -1 as i32,
        }
    }

    pub fn search_02(nums: Vec<i32>, target: i32) -> i32 {
        let mut left_index = 0;
        let mut right_index = nums.len() - 1;

        while left_index <= right_index {
            let mut mid = (left_index + right_index) / 2;
            if target == nums[mid] {
                return mid as i32;
            }

            // left sorted portion
            if nums[left_index] <= nums[mid] {
                if target > nums[mid] || target < nums[left_index]{
                    left_index = mid + 1;
                } else {
                    right_index = mid - 1;
                }
            } else {
                // right sorted portion
                if target < nums[mid] || target > nums[right_index]{
                    right_index = mid - 1;
                } else {
                    left_index = mid + 1;
                }
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_01() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let result = Solution::search_02(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_search_02() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let result = Solution::search_02(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_search_03() {
        let nums = vec![1];
        let target = 0;
        let result = Solution::search_02(nums, target);
        assert_eq!(result, -1);
    }
}
