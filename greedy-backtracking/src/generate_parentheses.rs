/*
#47

Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]

*/

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::back_track("".to_string(), n, 0)
    }

    fn back_track(s: String, open: i32, close: i32) -> Vec<String> {
        let mut res = vec![];

        if open == 0 && close == 0 {
            return vec![s];
        }

        if open > 0 {
            res.append(&mut Self::back_track(s.clone() + "(", open - 1, close + 1));
        }

        if close > 0 {
            res.append(&mut Self::back_track(s.clone() + ")", open, close - 1));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_parenthesis_01() {
        let n = 3;
        let result = Solution::generate_parenthesis(n);
        assert_eq!(
            result,
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn test_generate_parenthesis_02() {
        let n = 1;
        let result = Solution::generate_parenthesis(n);
        assert_eq!(result, vec!["()"]);
    }
}
