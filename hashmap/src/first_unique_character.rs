// #10
// Given a string s, find the first non-repeating character in it and return its index.
// If it does not exist, return -1.

use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_uniq_char_1() {
        let s = "leetcode".to_string();
        let ans = first_uniq_char_1(s);
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_first_uniq_char_2() {
        let s = "loveleetcode".to_string();
        let ans = first_uniq_char_1(s);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_first_uniq_char_3() {
        let s = "aabb".to_string();
        let ans = first_uniq_char_1(s);
        assert_eq!(ans, -1);
    }
}
