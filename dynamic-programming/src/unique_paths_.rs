/*
#28
You are given an m x n integer array grid.
There is a robot initially located at the top-left corner (i.e., grid[0][0]).
The robot tries to move to the bottom-right corner (i.e., grid[m-1][n-1]).
The robot can only move either down or right at any point in time.

An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.

Return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The testcases are generated so that the answer will be less than or equal to 2 * 109.

Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
Output: 2
Explanation: There is one obstacle in the middle of the 3x3 grid above.
There are two ways to reach the bottom-right corner:
1. Right -> Right -> Down -> Down
2. Down -> Down -> Right -> Right
*/

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let h = obstacle_grid.len();
        let w = obstacle_grid[0].len();
        let mut res_list = obstacle_grid.clone();

        if res_list[0][0] == 1 {
            return 0;
        }

        res_list[0][0] = 1;

        for i in 1..h {
            if res_list[i][0] == 1 {
                res_list[i][0] = 0;
            } else {
                res_list[i][0] = res_list[i - 1][0];
            }
        }

        for j in 1..w {
            if res_list[0][j] == 1 {
                res_list[0][j] = 0;
            } else {
                res_list[0][j] = res_list[0][j - 1];
            }
        }

        for i in 1..h {
            for j in 1..w {
                if res_list[i][j] == 1 {
                    res_list[i][j] = 0;
                    continue;
                }
                res_list[i][j] = res_list[i - 1][j] + res_list[i][j - 1];
            }
        }

        res_list[h - 1][w - 1]
    }
}

#[test]
fn test_unique_paths_with_obstacles() {
    let input: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    let res = Solution::unique_paths_with_obstacles(input);
    assert_eq!(res, 2);
}

#[test]
fn test_unique_paths_with_obstacles_2() {
    let input: Vec<Vec<i32>> = vec![vec![0, 1], vec![0, 0]];
    let res = Solution::unique_paths_with_obstacles(input);
    assert_eq!(res, 1);
}

#[test]
fn test_unique_paths_with_obstacles_3() {
    let input: Vec<Vec<i32>> = vec![vec![0]];
    let res = Solution::unique_paths_with_obstacles(input);
    assert_eq!(res, 1);
}
