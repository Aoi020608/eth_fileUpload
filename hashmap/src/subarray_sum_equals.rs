// Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.
// #11

use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let mut sum = 0;
    let mut map = HashMap::new();

    map.insert(0, 1);
    for i in nums {
        sum += i;
        count += map.get(&(sum - k)).unwrap_or(&0);
        map.insert(sum, map.get(&sum).unwrap_or(&0) + 1);
    }

    count
}

pub fn suarray_sum_1(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut presum = 0_i64;
    let mut cc = HashMap::new();
    cc.insert(0, 1);

    for n in nums {
        presum += n as i64;
        res += cc.get(&(presum - k as i64)).unwrap_or(&0);
        *cc.entry(presum).or_insert(0) += 1;
    }

    res
}

pub fn subarray_sum_2(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let mut count = 0;
    let mut map = HashMap::new();

    map.insert(0, 1);
    for n in nums {
        sum += n;
        count += map.get(&(sum - k)).unwrap_or(&0);
        map.insert(sum, map.get(&sum).unwrap_or(&0) + 1);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_uniq_char_1() {
        let s = vec![1, 1, 1];
        let k = 2;
        let ans = subarray_sum_2(s, k);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_first_uniq_char_2() {
        let s = vec![1, 2, 3];
        let k = 3;
        let ans = subarray_sum_2(s, k);
        assert_eq!(ans, 2);
    }
}
