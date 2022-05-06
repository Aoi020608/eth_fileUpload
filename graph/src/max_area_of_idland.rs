// #13

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let mut ans: i32 = 0;
        let height = grid.len();
        let width = grid[0].len();
        let mut visited = vec![vec![false; width]; height];
        for h in 0..height {
            for w in 0..width {
                if !visited[h][w] && grid[h][w] == 1 {
                    let count = Self::dfs(&grid, h as i32, w as i32, &mut visited);
                    if ans < count {
                        ans = count;
                    }
                }
            }
        }
        ans
    }
    pub fn dfs(grid: &Vec<Vec<i32>>, h: i32, w: i32, visited: &mut Vec<Vec<bool>>) -> i32 {
        if h < 0
            || h == grid.len().try_into().unwrap()
            || w < 0
            || w == grid[0].len().try_into().unwrap()
            || visited[h as usize][w as usize]
        {
            return 0 as i32;
        }
        visited[h as usize][w as usize] = true;
        if grid[h as usize][w as usize] == 0 {
            return 0 as i32;
        }
        let mut count = 1;
        count += Self::dfs(grid, h + 1, w, visited);
        count += Self::dfs(grid, h - 1, w, visited);
        count += Self::dfs(grid, h, w + 1, visited);
        count += Self::dfs(grid, h, w - 1, visited);
        count
    }

    #[allow(dead_code)]
    pub fn max_area_of_island_01(grid: Vec<Vec<i32>>) -> i32 {
        let height = grid.len();
        let width = grid[0].len();
        let mut ans = 0;
        let mut visited = vec![vec![false; width]; height];

        for h in 0..height {
            for w in 0..width {
                if grid[h][w] == 1 || !visited[h][w] {
                    let mut count = 0;
                    println!("{:?}", count);
                    count = Self::dfs_01(&grid, &mut visited, h, w);
                    if count > ans {
                        ans = count;
                    }
                }
            }
        }

        ans
    }

    fn dfs_01(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, h: usize, w: usize) -> i32 {
        if h >= grid.len() || w >= grid[0].len() || visited[h][w] {
            return 0;
        }

        visited[h][w] = true;

        if grid[h][w] == 0 {
            return 0;
        }

        let mut count = 1;

        count += Self::dfs_01(grid, visited, h + 1, w);

        if h > 0 {
            count += Self::dfs_01(grid, visited, h - 1, w);
        }

        count += Self::dfs_01(grid, visited, h, w + 1);

        if w > 0 {
            count += Self::dfs_01(grid, visited, h, w - 1);
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::max_area_of_idland::*;

    #[test]
    fn test_max_area_of_idlands() {
        let grid: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];

        let ans = Solution::max_area_of_island_01(grid);
        assert_eq!(ans, 6);
    }

    #[test]
    fn test_max_area_of_idlands_1() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];

        let ans = Solution::max_area_of_island_01(grid);
        assert_eq!(ans, 0);
    }
}
