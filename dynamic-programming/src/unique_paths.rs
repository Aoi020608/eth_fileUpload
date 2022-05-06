// #27

struct Solution;

impl Solution {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn unique_paths_1(m: i32, n: i32) -> i32 {
        let mut grid: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];

        for i in 0..n {
            grid[0][i as usize] = 1;
        }

        for j in 0..m {
            grid[j as usize][0] = 1;
        }

        for h in 1..m {
            for w in 1..n {
                grid[h as usize][w as usize] =
                    grid[h as usize - 1][w as usize] + grid[h as usize][w as usize - 1];
            }
        }

        grid[m as usize - 1][n as usize - 1] as i32
    }
}

#[test]
fn test_unique_paths() {
    let ans = Solution::unique_paths_1(3, 2);
    assert_eq!(ans, 3);
}

#[test]
fn test_unique_paths_1() {
    let ans = Solution::unique_paths_1(3, 7);
    assert_eq!(ans, 28);
}
