/*
#42
Given a string s, find the length of the longest substring without repeating characters.

*/

use std::cmp::max;
use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut sub_str_start: usize = 0;
        let mut longest = 0;
        let mut map: HashMap<char, usize> = HashMap::new();

        for (idx, c) in s.char_indices() {
            map.entry(c)
                .and_modify(|old_idx| {
                    if *old_idx >= sub_str_start {
                        // got a repetition
                        longest = max(longest, idx - sub_str_start);
                        sub_str_start = *old_idx + 1;
                    }
                    *old_idx = idx;
                })
                .or_insert(idx);
        }

        max(longest, s.len() - sub_str_start) as i32
    }

    pub fn length_of_longest_substring_1(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut indices = HashMap::new();
        let mut ret = 0;
        let mut i = 0;

        for j in 0..s.len() {
            match indices.insert(s[j], j) {
                Some(x) => i = i.max(x + 1),
                None => {}
            }
            ret = ret.max(j - i + 1);
        }

        ret as i32
    }

    pub fn length_of_longest_substring_2(s: String) -> i32 {
        let mut map = HashSet::new();
        let mut left: usize = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut res = 0;

        for r in 0..s.len() {
            while map.contains(&s[r]) {
                map.remove(&s[left]);
                left += 1;
            }

            map.insert(s[r]);
            res = max(res, r - left + 1);
        }

        return res as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        let s: String = "abcabcaa".to_string();
        let result = Solution::length_of_longest_substring_2(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_length_of_longest_substring_1() {
        let s: String = "bbbbbbbbb".to_string();
        let result = Solution::length_of_longest_substring_2(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_length_of_longest_substring_2() {
        let s: String = "pwwkew".to_string();
        let result = Solution::length_of_longest_substring_2(s);
        assert_eq!(result, 3);
    }
}
