// #7

use std::collections::HashMap;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut map = HashMap::new();
        let mut map2 = HashMap::new();
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
        return ans;
    }

    #[allow(dead_code)]
    pub fn intersection_01(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut map = HashMap::new();

        for num1 in nums1.iter() {
            map.insert(num1, 0);
        }

        for num2 in nums2.iter() {
            if let Some(freq) = map.get_mut(num2) {
                if freq == &0 {
                    ans.push(*num2);
                    *freq += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let ans = Solution::intersection_01(nums1, nums2);
        println!("{:?}", ans);
    }

    #[test]
    fn test_intersection_4() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let ans = Solution::intersection_01(nums1, nums2);
        println!("{:?}", ans);
    }
}
