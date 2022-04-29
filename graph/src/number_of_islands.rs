// #12
// Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.

// An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
// You may assume all four edges of the grid are all surrounded by water.

//Input: grid = [
//     ["1","1","1","1","0"],
//     ["1","1","0","1","0"],
//     ["1","1","0","0","0"],
//     ["0","0","0","0","0"]
//   ]
//   Output: 1

fn dimensions(grid: &[Vec<char>]) -> (usize, usize) {
    let n_rows = grid.len();
    let n_cols = grid.get(0).map(|col| col.len()).unwrap_or(0);
    (n_rows, n_cols)
}

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

fn explore_island(row: usize, col: usize, grid: &mut Vec<Vec<char>>) {
    let mut to_visit = vec![(row, col)];
    grid[row][col] = '0';
    let dimensions = dimensions(grid);
    while !to_visit.is_empty() {
        to_visit = to_visit
            .iter()
            .flat_map(|point| {
                neighbours(*point, dimensions)
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

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let (n_rows, n_cols) = dimensions(&grid);
    (0..n_rows)
        .flat_map(move |row| {
            (0..n_cols)
                .filter_map(|col| {
                    if grid[row][col] == '1' {
                        explore_island(row, col, &mut grid);
                        Some(())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .count() as i32
}

pub fn num_islands_1(grid: Vec<Vec<char>>) -> i32 {
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
                dfs(&grid, i, j, &mut used);
            }
        }
    }
    res
}

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
                dfs(&grid, i, j, &mut used);
            }
        }
    }

    res
}

fn dfs(grid: &Vec<Vec<char>>, i: usize, j: usize, used: &mut Vec<Vec<bool>>) {
    if i >= grid.len() || j >= grid[0].len() || used[i][j] {
        return;
    }

    used[i][j] = true;

    if grid[i][j] == '0' {
        return;
    }

    dfs(grid, i + 1, j, used);
    dfs(grid, i - 1, j, used);
    dfs(grid, i, j + 1, used);
    dfs(grid, i, j - 1, used);
}

fn dfs_2(grid: &Vec<Vec<char>>, i: usize, j: usize, used: &mut Vec<Vec<bool>>) {
    if i >= grid.len() || j >= grid[0].len() || used[i][j] {
        return;
    }

    used[i][j] = true;

    if grid[i][j] == '0' {
        return;
    }

    dfs_2(grid, i + 1, j, used);
    dfs_2(grid, i - 1, j, used);
    dfs_2(grid, i, j + 1, used);
    dfs_2(grid, i, j - 1, used);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_islands_1() {
        let mut grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let ans = num_islands(grid);
        assert_eq!(ans, 1);
    }
}
