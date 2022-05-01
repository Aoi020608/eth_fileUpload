/*
#25
Given an integer array nums, return the length of the longest strictly increasing subsequence.

A subsequence is a sequence that can be derived from an array by deleting some or no elements without
changing the order of the remaining elements. For example, [3,6,2,7] is a subsequence of the array [0,3,1,6,2,2,7].

Input: nums = [10,9,2,5,3,7,101,18]
Output: 4
Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.

*/

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len_nums = nums.len();

        let mut dp: Vec<i32> = vec![1; len_nums];
        let mut longest: i32 = 1;

        for hi in 1..len_nums {
            for lo in 0..hi {
                if nums[lo] < nums[hi] {
                    dp[hi] = std::cmp::max(dp[hi], dp[lo] + 1);
                }
            }

            longest = std::cmp::max(longest, dp[hi]);
        }

        longest
    }

    pub fn length_of_lis_1(nums: Vec<i32>) -> i32 {
        let mut new_set = Vec::new();

        for &i in nums.iter() {
            // To separate the num in increasing order, we have to sort
            if let Err(index) = new_set.binary_search(&i) {
                if index >= new_set.len() {
                    // Push the num to the piles
                    // LHS being a priority
                    new_set.push(i);
                } else {
                    // a new set
                    new_set[index] = i;
                }
            }
        }
        new_set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_increasing_subsequence::*;
    #[test]
    fn test_length_of_lis() {
        let ans = Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]);
        assert_eq!(ans, 4);
    }

    fn test_length_of_lis_2() {
        let ans = Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]);
        assert_eq!(ans, 4);
    }

    fn test_length_of_lis_3() {
        let ans = Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]);
        assert_eq!(ans, 1);
    }
}
