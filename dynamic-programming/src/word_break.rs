// #33

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut v: Vec<bool> = vec![false; s.len() + 1];
        v[0] = true;
        for i in 0..v.len() {
            if !v[i] {
                continue;
            }

            for word in &word_dict {
                if let Some(head) = s.get(i..i + word.len()) {
                    if head == word {
                        v[i + word.len()] = true;
                    }
                }
            }
        }

        println!("{:?}", v);

        v[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert_eq!(
            Solution::word_break(
                "leetfcode".to_string(),
                vec!["leet".to_string(), "code".to_string(),]
            ),
            true
        )
    }

    #[test]
    fn test_word_break_02() {
        assert_eq!(
            Solution::word_break(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string(),]
            ),
            true
        )
    }

    #[test]
    fn test_word_break_03() {
        assert_eq!(
            Solution::word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            false
        )
    }

    #[test]
    fn test_word_break_04() {
        let input = "applepenapple".to_string();
        println!("{:?}", input.get(0..5));
    }
}
