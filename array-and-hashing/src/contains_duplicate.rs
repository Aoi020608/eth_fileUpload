use std::collections::HashMap;

#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let contain = false;

    let mut map = HashMap::new();

    for num in nums {
        if map.contains_key(&num) {
            return true;
        }
        map.insert(num, true);
    }

    contain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let nums = vec![1, 2, 3, 1];
        assert!(contains_duplicate(nums));
    }

    #[test]
    fn test_does_not_contains_duplicate() {
        let nums = vec![1, 2, 3, 4];
        assert!(!contains_duplicate(nums));
    }
}
