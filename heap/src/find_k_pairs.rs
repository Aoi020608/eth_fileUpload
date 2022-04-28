//You are given two integer arrays nums1 and nums2 sorted in ascending order and an integer k.

// Define a pair (u, v) which consists of one element from the first array and one element from the second array.

// Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.

use std::collections::BinaryHeap;

pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    // nums1 = [1,7,11], nums2 = [2,4,6], k = 3
    // 1+2  1+4  1+6
    // 7+2  7+4  7+6
    // 11+2 11+4 11+6
    let mut res: Vec<Vec<i32>> = Vec::new();
    let (n1, n2) = (nums1.len(), nums2.len());
    // start of with the first column
    let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::from(
        (0..n1)
            .map(|x| (-nums1[x] - nums2[0], x, 0))
            .collect::<Vec<(i32, usize, usize)>>(),
    );
    println!("Binary Heap: {:?}", heap); // (-3, 0, 0), (-9, 1, 0), (-13, 2, 0)

    // pop off the lowest one
    while let Some((_, i, j)) = heap.pop() {
        res.push(vec![nums1[i], nums2[j]]);
        // end if reach k
        if res.len() == k as usize {
            break;
        }
        // push the right handside sum into the heap
        if j < n2 - 1 {
            heap.push((-nums1[i] - nums2[j + 1], i, j + 1));
        }
    }

    res
}

pub fn k_smallest_pairs_1(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let (n1, n2) = (nums1.len(), nums2.len());

    let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::from(
        (0..n1)
            .map(|x| (-nums1[x] - nums2[0], x, 0))
            .collect::<Vec<(i32, usize, usize)>>(),
    );

    res
}

pub fn k_smallest_pairs_2(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    if k == 0 || nums1.is_empty() || nums2.is_empty() {
        return ans;
    }

    let mut heap = BinaryHeap::<Vec<i32>>::new();
    for i in 0..std::cmp::min(k as usize, nums1.len()) {
        heap.push(vec![(-nums1[i] + nums2[0]), nums1[i], nums2[0], 0]);
    }

    while k > 0 && !heap.is_empty() {
        k -= 1;

        let cur = heap.pop().unwrap();
        ans.push(vec![cur[1], cur[2]]);
        if cur[3] == nums2.len() as i32 - 1 {
            continue;
        }
        heap.push(vec![
            -cur[1] - nums2[cur[3] as usize + 1],
            cur[1],
            nums2[cur[3] as usize + 1],
            cur[3] + 1,
        ]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_k_smallets_pairs_0() {
    //     let nums1 = vec![1, 7, 11];
    //     let nums2 = vec![2, 4, 6];
    //     let k = 3;

    //     let ans = k_smallest_pairs(nums1, nums2, k);
    //     // println!("{:?}", ans);
    // }

    #[test]
    fn tesst_k_smallest_pairs_2() {
        let nums1 = vec![1, 1, 2];
        let nums2 = vec![1, 2, 3];
        let k = 2;

        let ans = k_smallest_pairs(nums1, nums2, k);
        println!("{:?}", ans);
    }
}

//
