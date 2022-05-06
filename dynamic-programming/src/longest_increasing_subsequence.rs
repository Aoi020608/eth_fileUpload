// #25

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len_nums = nums.len();

        let mut dp: Vec<i32> = vec![1; len_nums];
        let mut longest: i32 = 1;

        for hi in 1..len_nums {
            for lo in 0..hi {
                if nums[lo] < nums[hi] {
                    dp[hi] = std::cmp::max(dp[hi], dp[lo] + 1);
                }
            }

            longest = std::cmp::max(longest, dp[hi]);
        }

        longest
    }

    #[allow(dead_code)]
    pub fn length_of_lis_1(nums: Vec<i32>) -> i32 {
        let mut new_set = Vec::new();

        for &i in nums.iter() {
            // To separate the num in increasing order, we have to sort
            if let Err(index) = new_set.binary_search(&i) {
                if index >= new_set.len() {
                    // Push the num to the piles
                    // LHS being a priority
                    new_set.push(i);
                } else {
                    // a new set
                    new_set[index] = i;
                }
            }
        }
        new_set.len() as i32
    }
    #[allow(dead_code)]
    pub fn length_of_lis_2(nums: Vec<i32>) -> i32 {
        let mut ret_list: Vec<i32> = vec![];

        for &num in nums.iter() {
            if let Err(e) = ret_list.binary_search(&num) {
                if e >= ret_list.len() {
                    ret_list.push(num);
                } else {
                    ret_list[e] = num;
                }
            }
        }

        ret_list.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_length_of_lis() {
        let ans = Solution::length_of_lis_2(vec![10, 9, 2, 5, 3, 7, 101, 18]);
        assert_eq!(ans, 4);
    }

    #[test]
    pub fn test_length_of_lis_2() {
        let ans = Solution::length_of_lis_2(vec![0, 1, 0, 3, 2, 3]);
        assert_eq!(ans, 4);
    }

    #[test]
    pub fn test_length_of_lis_3() {
        let ans = Solution::length_of_lis_2(vec![7, 7, 7, 7, 7, 7, 7]);
        assert_eq!(ans, 1);
    }
}
