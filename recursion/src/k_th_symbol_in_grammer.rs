/*
#39
We build a table of n rows (1-indexed). We start by writing 0 in the 1st row. Now in every subsequent row,
we look at the previous row and replace each occurrence of 0 with 01, and each occurrence of 1 with 10.

For example, for n = 3, the 1st row is 0, the 2nd row is 01, and the 3rd row is 0110.
Given two integer n and k, return the kth (1-indexed) symbol in the nth row of a table of n rows.

Input: n = 1, k = 1
Output: 0
Explanation: row 1: 0

*/

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn kth_grammar(k: i32) -> i32 {
        //https://doc.rust-lang.org/std/primitive.i32.html#method.count_ones
        (k - 1).count_ones() as i32 % 2
    }

    #[allow(dead_code)]
    pub fn kth_grammer_1(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }

        let parent = Self::kth_grammer_1(n - 1, (k as f64 / 2f64).ceil() as i32);
        let is_k_odd = k % 2 == 1;

        if parent == 1 {
            if is_k_odd {
                return 1;
            } else {
                return 0;
            }
        } else {
            if is_k_odd {
                return 0;
            } else {
                return 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kth_grammer() {
        let n = 1;
        let k = 1;
        let result = Solution::kth_grammer_1(n, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_kth_grammer_1() {
        let n = 2;
        let k = 1;
        let result = Solution::kth_grammer_1(n, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_kth_grammer_2() {
        let n = 2;
        let k = 2;
        let result = Solution::kth_grammer_1(n, k);
        assert_eq!(result, 1);
    }
}
