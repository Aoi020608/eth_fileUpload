/*
You are a professional robber planning to rob houses along a street.
Each house has a certain amount of money stashed. All houses at this place are arranged in a circle.
That means the first house is the neighbor of the last one.
Meanwhile, adjacent houses have a security system connected,
and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house,
return the maximum amount of money you can rob tonight without alerting the police.

Input: nums = [2,3,2]
Output: 3
Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
*/

use std::cmp::max;

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut cloned_nums = nums.clone();
        let mut stoled: Vec<i32> = Vec::with_capacity(len + 1);

        // compare first element and last element
        if cloned_nums[0] < cloned_nums[len - 1] {
            cloned_nums.reverse();
        }

        stoled.push(cloned_nums[0]);

        if len <= 1 {
            return cloned_nums[0];
        }

        if cloned_nums[0] > cloned_nums[1] {
            stoled.push(cloned_nums[0]);
        } else {
            stoled.push(cloned_nums[1]);
        }

        for index in 2..len {
            if cloned_nums[index] + stoled[index - 2] > stoled[index - 1] {
                stoled.push(cloned_nums[index] + stoled[index - 2]);
            } else {
                stoled.push(stoled[index - 1]);
            }
        }

        if len % 2 == 0 {
            return stoled[len - 1];
        } else {
            return stoled[len - 2];
        }
    }

    pub fn rob_1(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }

        Self::helper(&nums[0..n - 1]).max(Self::helper(&nums[1..n]))
    }

    fn helper(nums: &[i32]) -> i32 {
        let (mut h1, mut h2) = (0, 0);

        for n in nums {
            let temp = max(n + h1, h2);
            h1 = h2;
            h2 = temp;
        }
        h2
    }
}

#[test]
fn test_rob() {
    let mut input = vec![2, 3, 2];
    let ans = Solution::rob(input);

    assert_eq!(ans, 3);
}

#[test]
fn test_rob_1() {
    let mut input = vec![1, 2, 3, 1];
    let ans = Solution::rob(input);

    assert_eq!(ans, 4);
}

#[test]
fn test_rob_2() {
    let mut input = vec![1, 2, 3];
    let ans = Solution::rob(input);

    assert_eq!(ans, 3);
}

#[test]
fn test_rob_3() {
    let mut input = vec![1, 2, 1, 1];
    let ans = Solution::rob(input);

    assert_eq!(ans, 3);
}
