//Given an integer array nums and an integer k,
//return the k most frequent elements. You may return the answer in any order.

///Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]
use std::collections::*;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let frequent_elements: Vec<i32> = Vec::new();

    frequent_elements
}

pub fn top_k_frequent_1(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = Vec::with_capacity(k as usize);
    let mut occurence_map = HashMap::new();
    for num in &nums {
        *occurence_map.entry(num).or_insert(0) += 1;
    }
    let mut heap = BinaryHeap::new();
    for (num, o) in occurence_map {
        heap.push((o, num));
    }

    for _ in 0..k {
        let (_, num) = heap.pop().unwrap();
        res.push(*num);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_top_k_frequent_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 1;

        let ans = top_k_frequent_1(nums, k);
        assert_eq!(ans, [1]);
    }

    #[test]
    fn test_top_k_frequent_2() {
        let nums = vec![1, 1, 1, 2, 2, 3, 4, 4, 4];
        let k = 2;

        let ans = top_k_frequent_1(nums, k);
        assert_eq!(ans, [4, 1]);
    }
}
