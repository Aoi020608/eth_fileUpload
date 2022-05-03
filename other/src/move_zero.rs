/*
#48
Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

Note that you must do this in-place without making a copy of the array.

Input: nums = [0,1,0,3,12]
Output: [1,3,12,0,0]

*/

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_indexes = vec![];
        let mut i = 0;
        let mut nums_zero = 0;

        while i < nums.len() {
            if nums[i] == 0 {
                std::mem::swap(&mut nums[i], &mut nums[i + 1]);
                nums_zero += 1;
            }
        }

        for (index, num) in nums.iter().enumerate() {}
    }

    pub fn move_zeros(nums: &mut Vec<i32>) {
        let mut z = Vec::new();
        nums.retain(|x| {
            if *x != 0 {
                true
            } else {
                z.push(0);
                false
            }
        });
        &nums.append(&mut z);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
