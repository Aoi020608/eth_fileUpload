// #12

struct Solution;

impl Solution {
    #[allow(dead_code)]
    fn dimensions(grid: &[Vec<char>]) -> (usize, usize) {
        let n_rows = grid.len();
        let n_cols = grid.get(0).map(|col| col.len()).unwrap_or(0);
        (n_rows, n_cols)
    }

    #[allow(dead_code)]
    fn neighbours(
        point: (usize, usize),
        dimensions: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut neighbours = Vec::new();
        if point.0 > 0 {
            neighbours.push((point.0 - 1, point.1));
        }
        if point.0 + 1 < dimensions.0 {
            neighbours.push((point.0 + 1, point.1));
        }
        if point.1 > 0 {
            neighbours.push((point.0, point.1 - 1));
        }
        if point.1 + 1 < dimensions.1 {
            neighbours.push((point.0, point.1 + 1));
        }
        neighbours.into_iter()
    }

    #[allow(dead_code)]
    fn explore_island(row: usize, col: usize, grid: &mut Vec<Vec<char>>) {
        let mut to_visit = vec![(row, col)];
        grid[row][col] = '0';
        let dimensions = Self::dimensions(grid);
        while !to_visit.is_empty() {
            to_visit = to_visit
                .iter()
                .flat_map(|point| {
                    Self::neighbours(*point, dimensions)
                        .filter(|(row, col)| {
                            if grid[*row][*col] == '1' {
                                grid[*row][*col] = '0';
                                true
                            } else {
                                false
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
        }
    }

    #[allow(dead_code)]
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (n_rows, n_cols) = Self::dimensions(&grid);
        (0..n_rows)
            .flat_map(move |row| {
                (0..n_cols)
                    .filter_map(|col| {
                        if grid[row][col] == '1' {
                            Self::explore_island(row, col, &mut grid);
                            Some(())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .count() as i32
    }

    #[allow(dead_code)]
    pub fn num_islands_1(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let mut res = 0;
        let height = grid.len();
        let width = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
        for h in 0..height {
            for w in 0..width {
                if grid[h][w] == '1' && !visited[h][w] {
                    res += 1;
                    Self::dms(&grid, h, w, &mut visited);
                }
            }
        }
        res
    }

    fn dms(grid: &Vec<Vec<char>>, h: usize, w: usize, visited: &mut Vec<Vec<bool>>) {
        if h >= grid.len() || w >= grid[0].len() || visited[h][w] {
            return;
        }
        visited[h][w] = true;
        if grid[h][w] == '0' {
            return;
        }
        Self::dms(grid, h + 1, w, visited);
        Self::dms(grid, h - 1, w, visited);
        Self::dms(grid, h, w + 1, visited);
        Self::dms(grid, h, w - 1, visited);
    }

    #[allow(dead_code)]
    pub fn num_islands_2(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let hei = grid.len();
        let wid = grid[0].len();
        let mut used = vec![vec![false; wid]; hei];
        let mut res = 0;
        for i in 0..hei {
            for j in 0..wid {
                if grid[i][j] == '1' && !used[i][j] {
                    res += 1;
                    Self::dfs_2(&grid, i, j, &mut used);
                }
            }
        }
        res
    }
    fn dfs_2(grid: &Vec<Vec<char>>, i: usize, j: usize, used: &mut Vec<Vec<bool>>) {
        if i >= grid.len() || j >= grid[0].len() || used[i][j] {
            return;
        }
        used[i][j] = true;
        if grid[i][j] == '0' {
            return;
        }
        Self::dfs_2(grid, i + 1, j, used);
        Self::dfs_2(grid, i - 1, j, used);
        Self::dfs_2(grid, i, j + 1, used);
        Self::dfs_2(grid, i, j - 1, used);
    }

    #[allow(dead_code)]
    pub fn num_islands_03(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let mut res = 0;
        let height = grid.len();
        let width = grid[0].len();
        let mut visited = vec![vec![false; width]; height];

        for h in 0..height {
            for w in 0..width {
                if grid[h][w] == '1' && !visited[h][w] {
                    res += 1;
                    Self::dfs_03(&grid, &mut visited, h, w);
                }
            }
        }

        res
    }

    fn dfs_03(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, h: usize, w: usize) {
        if h >= grid.len() || w >= grid[0].len() || visited[h][w] {
            return;
        }

        visited[h][w] = true;

        if grid[h][w] == '0' {
            return;
        }

        Self::dfs_03(grid, visited, h + 1, w);
        if h > 0 {
            Self::dfs_03(grid, visited, h - 1, w);
        }
        Self::dfs_03(grid, visited, h, w + 1);
        if w > 0 {
            Self::dfs_03(grid, visited, h, w - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_islands_2() {
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let ans = Solution::num_islands_03(grid);
        assert_eq!(ans, 1);
    }
}
