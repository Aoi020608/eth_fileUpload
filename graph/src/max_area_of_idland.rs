// You are given an m x n binary matrix grid.
// An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.)
//You may assume all four edges of the grid are surrounded by water.
// The area of an island is the number of cells with a value 1 in the island.
// Return the maximum area of an island in grid. If there is no island, return 0.

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
                let count = dfs(&grid, h as i32, w as i32, &mut visited);
                if ans < count {
                    ans = count;
                }
            }
        }
    }

    ans
}

pub fn dfs(grid: &Vec<Vec<i32>>, h: i32, w: i32, visited: &mut Vec<Vec<bool>>) -> i32 {
    // if h as usize <= grid.len() || w as usize <= grid[0].len() || visited[h as usize][w as usize] {
    //     return 0 as i32;
    // }
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

    count += dfs(grid, h + 1, w, visited);
    count += dfs(grid, h - 1, w, visited);
    count += dfs(grid, h, w + 1, visited);
    count += dfs(grid, h, w - 1, visited);

    count
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

        let ans = max_area_of_island(grid);
        assert_eq!(ans, 6);
    }

    #[test]
    fn test_max_area_of_idlands_1() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];

        let ans = max_area_of_island(grid);
        assert_eq!(ans, 0);
    }
}
