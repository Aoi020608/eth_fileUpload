use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = Vec::<i32>::new();
        let mut map: HashMap<i32, u32> = HashMap::new();
        let mut clo_nums = nums.clone();

        for num in clo_nums {
            map.entry(num).and_modify(|count| *count += 1).or_insert(0);
        }

        let clo_map = map.clone();
        let mut val_arrs: Vec<u32> = clo_map.into_iter().map(|(_, val)| val).collect();
        val_arrs.sort();
        val_arrs.reverse();
        // let map_len = clo_map.len();

        for (key, val) in map.into_iter() {
            for val_arr in val_arrs.iter() {
                if val == *val_arr {
                    arr.push(key);
                    break;
                }
            }
        }

        // map.into_iter().map(|(key, val)| val);
        arr.split_at(k as usize).0.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2i32;

        assert_eq!(Solution::top_k_frequent(nums, k), [1, 2]);
    }
}
