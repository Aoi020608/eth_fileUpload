/*
You are a professional robber planning to rob houses along a street.
Each house has a certain amount of money stashed,
the only constraint stopping you from robbing each of them is that adjacent
houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house,
return the maximum amount of money you can rob tonight without alerting the police.

Input: nums = [1,2,3,1]
Output: 4
Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
Total amount you can rob = 1 + 3 = 4.

Input [2, 1, 1, 2]
Output 4


*/

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        // let ret_list = vec![];
        let mut ans_ev = 0;
        let mut ans_odd = 0;

        if len == 0 {
            return 0;
        }

        if len == 1 {
            return nums[0];
        }

        for i in 0..nums.len() {
            // if i % 2 == 0 {
            //     ans_ev += nums[i];
            // } else {
            //     ans_odd += nums[i];
            // }
        }

        if ans_ev > ans_odd {
            return ans_ev;
        } else {
            return ans_odd;
        }
    }

    pub fn rob_1(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(a, b), x| (a.max(b + x), a))
            .0
    }

    pub fn rob_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_stolen = Vec::with_capacity(n + 1);
        max_stolen.push(nums[0]);

        if n == 1 {
            return nums[0];
        }

        if nums[0] > nums[1] {
            max_stolen.push(nums[0]);
        } else {
            max_stolen.push(nums[1]);
        }

        for index in 2..n {
            if nums[index] + max_stolen[index - 2] > max_stolen[index - 1] {
                max_stolen.push(nums[index] + max_stolen[index - 2]);
            } else {
                max_stolen.push(max_stolen[index - 1]);
            }
        }

        return max_stolen[n - 1];
    }
}

#[test]
fn test_rob() {
    let input = vec![1, 2, 3, 1];
    let ans = Solution::rob_2(input);

    assert_eq!(ans, 4);
}

#[test]
fn test_rob_1() {
    let input = vec![2, 7, 9, 3, 1];
    let ans = Solution::rob_2(input);

    assert_eq!(ans, 12);
}
