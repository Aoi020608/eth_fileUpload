/*
#27
Dynamic Programming

There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]).
The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]).
The robot can only move either down or right at any point in time.

Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The test cases are generated so that the answer will be less than or equal to 2 * 109.

Input: m = 3, n = 7
Output: 28

*/

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let mut paths = vec![vec![0; n]; m];

        for i in 0..m {
            paths[i][0] = 1;
        }

        for j in 0..n {
            paths[0][j] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                paths[i][j] = paths[i - 1][j] + paths[i][j - 1];
            }
        }

        return paths[m - 1][n - 1] as i32;
    }

    pub fn unique_paths_1(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }

        return Self::unique_paths_1(m - 1, n) + Self::unique_paths_1(m, n - 1);
    }
}

#[test]
fn test_unique_paths() {
    let ans = Solution::unique_paths_1(3, 2);
    assert_eq!(ans, 3);
}

fn test_unique_paths_1() {
    let ans = Solution::unique_paths_1(3, 7);
    assert_eq!(ans, 21);
}
