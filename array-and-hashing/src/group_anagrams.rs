use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut ret_vec = Vec::new();
    let clone_strs = strs.clone();

    for _str in clone_strs {
        let _map: HashMap<&str, &str> = HashMap::new();
    }

    ret_vec.push(vec!["".to_string()]);

    ret_vec
}

pub fn group_anagrams_solution(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut h = HashMap::<String, Vec<String>>::new();

    for s in strs {
        h.entry(get_sorted(&s)).or_default().push(s)
    }

    h.into_iter().map(|(_, v)| v).collect::<Vec<Vec<String>>>()
}

fn get_sorted(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    chars.iter().collect::<String>()
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
            group_anagrams_solution(inputs),
            [vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        );
    }
}
