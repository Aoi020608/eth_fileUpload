//Given two integer arrays nums1 and nums2,
//return an array of their intersection.
//Each element in the result must be unique and you may return the result in any order.

// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2]

//Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [9,4]
// Explanation: [4,9] is also accepted.

use std::collections::HashMap;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut map2 = HashMap::new();
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..nums1.len() {
        if !map.contains_key(&nums1[i]) {
            map.insert(nums1[i], true);
        } else {
            continue;
        }
    }

    for i in 0..nums2.len() {
        if map.contains_key(&nums2[i]) {
            map2.insert(nums2[i], true);
        } else {
            continue;
        }
    }

    ans = map2.into_keys().collect();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let ans = intersection(nums1, nums2);
        println!("{:?}", ans);
    }

    #[test]
    fn test_intersection_4() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let ans = intersection(nums1, nums2);
        println!("{:?}", ans);
    }
}
