/*
#35
Given a sorted array of distinct integers and a target value,
return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.

Input: nums = [1,3,5,6], target = 5
Output: 2

*/

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Err(e) => return e as i32,
            Ok(index) => return index as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_insert() {
        let nums = vec![1, 2, 5, 6];
        let target = 5;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_search_insert_01() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_search_insert_02() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 4);
    }
}
