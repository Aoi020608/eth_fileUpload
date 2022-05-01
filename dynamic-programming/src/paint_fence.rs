/*
#24
There is a fence with n posts, each post can be painted with one of the k colors.

You have to paint all the posts such that no more than two adjacent fence posts have the same color.

Return the total number of ways you can paint the fence.

Note:
n and k are non-negative integers.

Input: n = 3, k = 2
Output: 6

           post1  post2  post3
 -----      -----  -----  -----
   1         c1     c1     c2
   2         c1     c2     c1
   3         c1     c2     c2
   4         c2     c1     c1
   5         c2     c1     c2
   6         c2     c2     c1

*/

struct Solution;

impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n == 0 || k == 0 {
            return 0;
        }

        if n == 1 {
            return k;
        }

        if k == 1 {
            return 1;
        }

        let mut same = k;
        let mut diff = k * (k - 1);

        for _ in 2..n {
            let t_same = same;
            let t_diff = diff;
            same = t_diff;
            diff = (t_same + t_diff) * (k - 1);
        }

        same + diff
    }
}

#[cfg(test)]
mod tests {
    use crate::paint_fence::*;
    #[test]
    fn test_num_ways() {
        let result = Solution::num_ways(1, 2);
        println!("Result: {:?}", result);
        // assert_eq!(result, 6);
    }
}
