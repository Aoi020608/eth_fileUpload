use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut arr = Vec::<i32>::new();
    // let clo_nums = nums.clone();
    // let count_arr: [i32; clo_nums.len()];

    arr.split_at(k as usize).0.to_vec()
}

pub fn top_k_frequent_2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for n in nums {
        let val = map.entry(n).or_insert(0);
        *val += 1;
    }
    let mut res: Vec<_> = map.iter_mut().collect();
    res.sort_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap());
    res.into_iter()
        .rev()
        .take(k as usize)
        .map(|(k, _)| *k)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2i32;

        assert_eq!(top_k_frequent_2(nums, k), [1, 2]);
    }
}
