use std::collections::{HashMap, HashSet};

#[warn(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool {
    let mut map: HashMap<char, i8> = HashMap::new();
    // let mut set: HashSet = HashSet::new();

    if s.len() != t.len() {
        return false;
    }

    for ch in s.chars() {
        map.entry(ch).and_modify(|count| *count += 1).or_insert(0);
    }

    for ch in t.chars() {
        map.entry(ch).and_modify(|count| *count -= 1).or_insert(-1);
    }

    for val in map.values() {
        if *val == -1 {
            return false;
        }
    } 

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        let s = "anagram".to_string();
        let t = "naagram".to_string();

        assert!(is_anagram(s, t));
    }

    #[test]
    fn test_is_anagram2() {
        let s = "rat".to_string();
        let t = "car".to_string();

        assert!(!is_anagram(s, t));
    }
}
