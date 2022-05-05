// #9

use std::collections::HashMap;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut map: HashMap<String, i32> = HashMap::new();
        for email in emails.into_iter() {
            let trimmed = email.trim().split('@').collect::<Vec<&str>>();
            let (local, domain) = (trimmed[0], trimmed[1]);
            let local = local
                .chars()
                .filter(|&x| x != '.')
                .collect::<Vec<char>>()
                .into_iter()
                .collect::<String>();
            if local.contains('+') {
                let local = *local.split('+').collect::<Vec<&str>>().first().unwrap();
                let key = local.to_string() + "@" + domain;
                if !map.contains_key(&key) {
                    let count = map.entry(key).or_insert(0);
                    *count += 1;
                }
            } else {
                let key = local + "@" + domain;
                if !map.contains_key(&key) {
                    let count = map.entry(key).or_insert(0);
                    *count += 1;
                }
            }
        }
        map.len() as i32
    }

    #[allow(dead_code)]
    pub fn num_unique_emails_01(emails: Vec<String>) -> i32 {
        let mut map = HashMap::new();

        for email in emails.into_iter() {
            let trimmed = email.trim().split('@').collect::<Vec<&str>>();
            let (local, domain) = (trimmed[0], trimmed[1]);

            let local = local
                .chars()
                .filter(|&x| x != '.')
                .collect::<Vec<char>>()
                .into_iter()
                .collect::<String>();
            if local.contains('+') {
                let local = *local.split('+').collect::<Vec<&str>>().first().unwrap();
                let key = local.to_string() + "@" + domain;
                if !map.contains_key(&key) {
                    *map.entry(key).or_insert(0) += 1;
                }
            } else {
                let key = local.to_string() + "@" + domain;
                if !map.contains_key(&key) {
                    *map.entry(key).or_insert(0) += 1;
                }
            }
        }

        map.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_unique_emails() {
        let emails = vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string(),
        ];
        let ans = Solution::num_unique_emails_01(emails);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_num_unique_emails_1() {
        let emails = vec![
            "a@leetcode.com".to_string(),
            "b@leetcode.com".to_string(),
            "c@leetcode.com".to_string(),
        ];
        let ans = Solution::num_unique_emails_01(emails);
        assert_eq!(ans, 3);
    }
}
