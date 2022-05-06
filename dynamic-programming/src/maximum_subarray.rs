// #26

use std::cmp;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = i32::MIN;
        let mut max = 0;
        if nums.len() > 1 {
            for (_, &v) in nums.iter().enumerate() {
                max = cmp::max(max + v, v);
                sum = cmp::max(sum, max);
            }
            return sum;
        } else {
            return nums[0];
        }
    }

    #[allow(dead_code)]
    pub fn max_sub_array_1(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let len_nums = nums.len();
        let mut result = nums;

        for i in 1..len_nums {
            let curr_num = result.get(i).unwrap();
            let prev_num = result.get(i - 1).unwrap();

            if curr_num + prev_num >= *curr_num {
                if curr_num + prev_num > ans {
                    ans = curr_num + prev_num;
                }
                *result.get_mut(i).unwrap() = curr_num + prev_num;
            }
        }

        println!("Answer: {:?}", ans);
        println!("Result: {:?}", result);
        ans
    }

    #[allow(dead_code)]
    pub fn max_sub_array_02(nums: Vec<i32>) -> i32 {
        let len_nums = nums.len();
        let mut result = nums;

        for i in 1..len_nums {
            let curr_val = result.get(i).unwrap();
            let prev_val = result.get(i - 1).unwrap();

            if curr_val + prev_val >= *curr_val {
                *result.get_mut(i).unwrap() = curr_val + prev_val;
            }
        }

        *result.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::maximum_subarray::*;
    #[test]
    fn test_num_ways() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = Solution::max_sub_array_1(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_num_ways_1() {
        let nums = vec![1];
        let result = Solution::max_sub_array_1(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_num_ways_2() {
        let nums = vec![5, 4, -1, 7, 8];
        let result = Solution::max_sub_array_1(nums);
        // println!("Result: {:?}", result);
        assert_eq!(result, 23);
    }
}
