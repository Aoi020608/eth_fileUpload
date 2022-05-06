// #28

struct Solution;

impl Solution {
    #[allow(dead_code)]
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
