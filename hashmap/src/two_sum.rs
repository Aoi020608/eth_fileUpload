// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let mut map = HashMap::new();
    let mut diff: i32;

    for i in 0..nums.len() {
        diff = target - nums[i];
        if map.contains_key(&diff) {
            let index = map[&diff];
            ans.push(index);
            ans.push(i as i32);
        } else {
            map.insert(nums[i], i as i32);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let ans = two_sum(nums, target);
        println!("{:?}", ans);
        // assert_eq!(result, 4);
    }
}
