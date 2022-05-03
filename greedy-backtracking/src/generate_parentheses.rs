/*
#47

Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]

n = 3 
- 3 open, 3 close
- close < open
-  

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

    #[allow(dead_code)]
    pub fn generate_parenthesis_01(n: i32) -> Vec<String> {
        // only add open paranthesis if open < n
        // only add a closing paranthesis if closed < open
        // valid if open == close == n

        let mut stack = vec![];
        let mut res = vec![];

        Self::backtrack(0, 0, n, &mut res, &mut stack);

        res

    }

    fn backtrack(open_num: i32, close_num: i32, n: i32, res: &mut Vec<String>, stack: &mut Vec<String>) {
        if open_num == n && close_num == n {
            // res.append("".)
            return res.append(stack);
        } 

        if open_num < n {
            let mut open = vec!["(".to_string()];
            stack.append(open.as_mut());
            Self::backtrack(open_num + 1, close_num, n, res, stack);
            stack.pop();
        }

        if close_num < open_num {
            let mut close = vec![")".to_string()];
            stack.append(close.as_mut());
            Self::backtrack(open_num, close_num + 1, n, res, stack);
            stack.pop();
        }
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_parenthesis_01() {
        let n = 3;
        let result = Solution::generate_parenthesis_01(n);
        assert_eq!(
            result,
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn test_generate_parenthesis_02() {
        let n = 1;
        let result = Solution::generate_parenthesis_01(n);
        assert_eq!(result, vec!["()"]);
    }
}

// 
