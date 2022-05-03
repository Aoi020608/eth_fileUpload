/*
#44
Given an array nums of distinct integers, return all the possible permutations.
You can return the answer in any order.

Input: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

*/

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut p: Vec<i32> = vec![];

        Self::_permute(nums, &mut res, &mut p);

        res
    }

    fn _permute(nums: Vec<i32>, res: &mut Vec<Vec<i32>>, p: &mut Vec<i32>) {
        if nums.is_empty() {
            res.push(p.to_vec());
            return;
        }

        for (i, &value) in nums.iter().enumerate() {
            p.push(value);
            let mut tmp = nums.clone();
            tmp.remove(i);
            Self::_permute(tmp, res, p);
            println!("Index: {:?}, Value: {:?}", i, value);
            p.pop();
        }
    }

    pub fn permute_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut clone_nums = VecDeque::from(nums.clone());

        if clone_nums.len() == 1 {
            return vec![Vec::from(clone_nums.clone())];
        }

        // [1, 2, 3]
        for i in 0..clone_nums.len() {
            let n = clone_nums.pop_front().unwrap();
            let mut perms = Self::permute_1(Vec::from(clone_nums.clone()));

            for perm in perms.iter_mut() {
                perm.push(n);
            }
            println!("I: {:?}, Perms: {:?}", i, perms);
            result.extend(perms);
            clone_nums.push_back(n);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_permute() {
        let nums = vec![1, 2, 3];
        let result = Solution::permute_1(nums);
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    // #[test]
    // fn test_permute_1() {
    //     let nums = vec![0, 1];
    //     let result = Solution::permute_1(nums);
    //     assert_eq!(result, vec![vec![0, 1], vec![1, 0],]);
    // }

    // #[test]
    // fn test_permute_2() {
    //     let nums = vec![1];
    //     let result = Solution::permute(nums);
    //     assert_eq!(result, vec![vec![1]]);
    // }
}
