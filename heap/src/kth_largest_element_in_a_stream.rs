// #3

// use rust_heap::BoundedBinaryHeap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    k: i32,
    heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    /*
    #[allow(dead_code)]
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        // minHeap with K largest intergers
        let heap = Self::add_to_heap(nums, k);

        KthLargest { k, min_heap: heap }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, val: i32) -> i32 {
        // let mut new_heap = self.min_heap.clone();

        if self.min_heap.len() < self.k as usize {
            self.min_heap.push(val);
        } else {
            let top_val = self.min_heap.peek().unwrap();
            if top_val < &val {
                self.min_heap.pop();
                self.min_heap.push(val);
            }
        }

        // self.min_heap.push(val);

        // if self.min_heap.len() < self.k as usize {
        //     self.min_heap.pop().unwrap();
        // }
        // return Vec::from(self.min_heap.clone())[0];
        let top_val = self.min_heap.peek().unwrap();
        return *top_val;
    }

    #[allow(dead_code)]
    fn add_to_heap(nums: Vec<i32>, k: i32) -> BoundedBinaryHeap<i32> {
        let mut heap = BoundedBinaryHeap::new(k as usize);

        for num in nums.iter() {
            if heap.len() < k as usize {
                heap.push(*num);
            } else {
                let top_val = heap.peek().unwrap();
                if top_val < &num {
                    heap.pop();
                    heap.push(*num);
                }
            }
        }

        heap
    }
    */

    #[allow(dead_code)]
    pub fn new_01(k: i32, nums: Vec<i32>) -> Self {
        let mut _self = KthLargest {
            k,
            heap: BinaryHeap::with_capacity(k as usize + 1),
        };

        for n in nums {
            _self.add_01(n);
        }

        _self
    }

    #[allow(dead_code)]
    pub fn add_01(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k as usize {
            self.heap.pop();
        }

        if let Some(Reverse(x)) =  self.heap.peek() {
            return *x;
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kth_largest_1() {
        let nums = vec![4, 5, 8, 2];
        let k = 3;

        let mut kth = KthLargest::new_01(k, nums);

        let ans_3 = KthLargest::add_01(&mut kth, 3);
        assert_eq!(ans_3, 4);

        let ans_5 = KthLargest::add_01(&mut kth, 5);
        assert_eq!(ans_5, 5);
    }
}
