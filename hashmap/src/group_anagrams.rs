//Given an array of strings strs, group the anagrams together. You can return the answer in any order.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
// typically using all the original letters exactly once.

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = vec![];
    let mut m: HashMap<[u8; 26], usize> = HashMap::new();
    let mut i = 0;

    for _str in strs.iter() {
        let mut s: [u8; 26] = [0; 26];
        for c in _str.chars() {
            let ci = c as usize - 'a' as usize;
            s[ci] += 1;
        }
        match m.get(&s) {
            Some(j) => {
                res[*j].push(_str.to_string());
            }
            None => {
                m.insert(s, i);
                if res.len() < i + 1 {
                    res.push(vec![]);
                }
                res[i].push(_str.to_string());
                i += 1;
            }
        }
    }

    res
}

pub fn group_anagrams_2(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for s in strs.into_iter() {
        let mut key = [0; 26];
        for ch in s.chars() {
            key[(ch as u32 - 'a' as u32) as usize] += 1;
        }
        println!("Key: {:?}", key);
        let arr = map.entry(key).or_insert(Vec::new());
        arr.push(s);
    }
    println!("Map: {:?}", map);
    map.into_iter().map(|(_, v)| v).collect()
}

pub fn group_anagrams_3(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for s in strs.into_iter() {
        let mut key = [0; 26];
        for ch in s.chars() {
            key[(ch as usize - 'a' as usize) as usize] += 1;
        }
        let arr = map.entry(key).or_insert(Vec::new());
        arr.push(s);
    }

    map.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group_anagrams_1() {
        let _str: Vec<String> = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        // let nums2 = vec![];
        let ans = group_anagrams_2(_str);
        println!("{:?}", ans);
    }

    #[test]
    fn test_group_anagrams_2() {
        let _str: Vec<String> = vec!["a".to_string()];
        // println!("{:?}", _str);
        let ans = group_anagrams_2(_str);
        assert_eq!(ans, vec![["a".to_string()]]);
    }
    // Key: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // Map: {[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]: ["a"]}
}

// How do we know
