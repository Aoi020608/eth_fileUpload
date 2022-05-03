/*
#46
Given an array of distinct integers candidates and a target integer target,
return a list of all unique combinations of candidates where the chosen numbers sum to target.
You may return the combinations in any order.

The same number may be chosen from candidates an unlimited number of times.
Two combinations are unique if the frequency of at least one of the chosen numbers is different.

It is guaranteed that the number of unique combinations that sum up to target is less than 150 combinations for the given input.

Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]

*/

struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut clone_candidates = candidates.clone();
        let mut combination: Vec<i32> = vec![];
        clone_candidates.sort();

        Self::_combination_sum(&mut clone_candidates, target, &mut res, &mut combination, 0);
        res
    }

    fn _combination_sum(
        candidates: &mut Vec<i32>,
        target: i32,
        res: &mut Vec<Vec<i32>>,
        combination: &mut Vec<i32>,
        cursor: usize,
    ) {
        if target == 0 {
            res.push(combination.to_vec());
            return;
        }

        let mut i = cursor;

        while i < candidates.len() && target >= candidates[i] {
            combination.push(candidates[i]);
            Self::_combination_sum(candidates, target - candidates[i], res, combination, i);
            combination.pop();
            i = i + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_combination_works() {
        let candidates: Vec<i32> = vec![2, 3, 6, 7];
        let target = 7;
        let result = Solution::combination_sum(candidates, target);
        assert_eq!(result, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test_combination_works_1() {
        let candidates: Vec<i32> = vec![2, 3, 5];
        let target = 8;
        let result = Solution::combination_sum(candidates, target);
        assert_eq!(result, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }
}
