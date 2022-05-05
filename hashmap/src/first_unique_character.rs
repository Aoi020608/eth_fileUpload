// #10

use std::collections::HashMap;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn first_uniq_char(s: String) -> i32 {
        // let mut map = HashMap::new();
        let characters = s.chars().collect::<Vec<char>>();
        let mut ans = -1;
        println!("Characters: {:?}", characters);
        for i in 0..characters.len() {
            for j in i + 1..characters.len() {
                if characters[i] == characters[j] {
                    break;
                }
                if j == characters.len() - 1 {
                    ans = i as i32;
                }
            }
            if i as i32 != -1 {
                break;
            }
        }
        ans
    }
    #[allow(dead_code)]
    pub fn first_uniq_char_1(s: String) -> i32 {
        let mut m = HashMap::new();
        for i in s.chars() {
            *m.entry(i).or_insert(0) += 1;
        }
        for (i, v) in s.chars().enumerate() {
            if m.get(&v).unwrap() == &1 {
                return i as i32;
            }
        }
        -1
    }

    #[allow(dead_code)]
    pub fn first_uniq_char_02(s: String) -> i32 {
        let mut index = -1 as isize;
        let mut map = HashMap::new();

        for chr in s.chars() {
            *map.entry(chr).or_insert(0) += 1;
        }

        for (i, chr) in s.chars().enumerate() {
            if let Some(count) = map.get(&chr) {
                println!("Count: {:?}", count);
                if count == &1 {
                    index = i as isize;
                    break;
                }
            }
        }

        index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_uniq_char_1() {
        let s = "leetcode".to_string();
        let ans = Solution::first_uniq_char_02(s);
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_first_uniq_char_2() {
        let s = "loveleetcode".to_string();
        let ans = Solution::first_uniq_char_02(s);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_first_uniq_char_3() {
        let s = "aabb".to_string();
        let ans = Solution::first_uniq_char_02(s);
        assert_eq!(ans, -1);
    }
}
