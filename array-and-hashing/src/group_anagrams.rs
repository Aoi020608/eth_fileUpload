pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut ret_vec = Vec::new();
    ret_vec.push(vec!["".to_string()]);

    ret_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let inputs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        assert_eq!(
            group_anagrams(inputs),
            [vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        );
    }
}
