/*
Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

A subsequence of a string is a new string that is formed from the original string
by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters.
(i.e., "ace" is a subsequence of "abcde" while "aec" is not).

Input: s = "abc", t = "ahbgdc"
Output: true

*/

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut is_valid = true;
        let mut s_vec = s.chars().collect::<Vec<char>>();
        let mut t_vec = t.chars().collect::<Vec<char>>();
        let mut indexes = Vec::new();
        if s_vec.is_empty() {
            return true;
        }
        if t_vec.is_empty() {
            return false;
        }

        for (s_index, s_value) in s_vec.iter().enumerate() {
            let mut highest = 0;
            let mut times = 0;
            for (t_index, t_value) in t_vec.iter().enumerate() {
                if s_value == t_value && t_index >= highest {
                    if times > 0 {
                        indexes.pop();
                    }
                    indexes.push(t_index);
                    highest = t_index;
                    times += 1;
                }
            }
        }

        if indexes.len() <= 0 {
            return false;
        }

        println!("Indexes: {:?}", indexes);

        for i in 0..indexes.len() - 1 {
            if indexes[i] >= indexes[i + 1] || indexes.len() < s_vec.len() {
                is_valid = false;
                break;
            }
        }

        is_valid
    }

    pub fn is_subsequence_1(s: String, t: String) -> bool {
        let mut iter = t.chars();

        for c in s.chars() {
            match iter.find(|&p| p == c ) {
                Some(_) => (),
                None => return false
            }
        }

        true
    }

    pub fn is_subsequence_2(s: String, t: String) -> bool {
        let mut s_vec = s.chars().collect::<Vec<char>>();
        let mut t_vec = t.chars().collect::<Vec<char>>();
        
        for t_value in t_vec {
            if let Some(&first) = s_vec.first() {
                if first == t_value {
                    s_vec.remove(0);
                }
            }
        }

        if s_vec.is_empty() {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_subsequence() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        let result = Solution::is_subsequence_2(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_subsequence_01() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        let result = Solution::is_subsequence_2(s, t);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_subsequence_02() {
        let s = "bb".to_string();
        let t = "ahbgdc".to_string();
        let result = Solution::is_subsequence_2(s, t);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_subsequence_03() {
        let s = "ab".to_string();
        let t = "baab".to_string();
        let result = Solution::is_subsequence_2(s, t);
        assert_eq!(result, true);
    }
}
