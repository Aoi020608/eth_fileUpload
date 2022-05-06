// #14

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for word in word_list.iter() {
            for i in 0..word.len() {
                let s = word.as_str();
                let key = (&s[0..i]).to_string() + "*" + &s[i + 1..];
                // if let Some(v) = map.get_mut(&key) {
                //     v.push(s.to_string());
                // } else {
                //     map.insert(key, vec![s.to_string()]);
                // }
                let arr = map.entry(key).or_insert(vec![s.to_string()]);
                arr.push(s.to_string());
            }
        }

        let mut set: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();

        set.insert(begin_word.to_string());
        queue.push_back((begin_word, 1));

        while let Some(front) = queue.pop_front() {
            let s = front.0.as_str();
            for i in 0..s.len() {
                let key = (&s[0..i]).to_string() + "*" + &s[i + 1..];
                if let Some(v) = map.get(&key) {
                    for next in v.into_iter() {
                        if *next == end_word {
                            return front.1 + 1;
                        }
                        if !set.contains(next) {
                            set.insert(next.to_string());
                            queue.push_back((next.to_string(), front.1 + 1));
                        }
                    }
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ladder_length() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
            "cog".to_string(),
        ];
        let ans = Solution::ladder_length(begin_word, end_word, word_list);
        assert_eq!(ans, 5);
    }

    #[test]
    fn test_ladder_length_01() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
        ];
        let ans = Solution::ladder_length(begin_word, end_word, word_list);
        assert_eq!(ans, 0);

        // let c = "begin_word".chars().collect::<Vec<char>>();
        // println!("{:?}", &c[..2]);
    }
}
