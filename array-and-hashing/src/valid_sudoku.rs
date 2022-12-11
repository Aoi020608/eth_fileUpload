use std::{collections::HashSet};

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut columns: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut squares: Vec<Vec<HashSet<char>>> = vec![vec![HashSet::new(); 9]; 9];

    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == '.' {
                continue;
            }

            if !rows[r].insert(board[r][c])
                || !columns[c].insert(board[r][c])
                || !squares[r / 3][c / 3].insert(board[r][c])
            {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        let input: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(is_valid_sudoku(input), true)
    }
}
