/*
The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: 
(you may want to display this pattern in a fixed font for better legibility)

*/

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows: usize = num_rows as usize;
        if num_rows == 1 || num_rows > s.len() {
            return s;
        }

        let mut zigzag = vec![vec![]; num_rows];
        let mut row = 0;
        let mut step = 1;

        for c in s.chars() {
            zigzag[row].push(c);

            if row == 0 {
                step = 1;
            } else if row == num_rows - 1 {
                step = -1;
            }

            row = (row as isize + step) as usize;
        }

        return zigzag.iter().fold("".to_string(), |s, v| format!("{}{}", s, v.iter().collect::<String>()));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PAHNAPLSIIGYIR".to_string());
    }

    #[test]
    fn test_convert_01() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PINALSIGYAHRPI".to_string());
    }
}
