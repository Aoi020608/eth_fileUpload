use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut res: Vec<i32> = Vec::new();
    let mut temp_arr: Vec<i32> = Vec::new();
    // let mut prev_num = i32::MIN;

    let mut clo_nums = nums.clone();
    clo_nums.sort();

    for num in clo_nums {
        // prev_num = num.clone();

        if temp_arr.is_empty() {
            temp_arr.push(num.clone());
            continue;
        }

        if temp_arr.ends_with(&[num - 1]) {
            temp_arr.push(num);
        } else {
            res.push(temp_arr.len() as i32);
            temp_arr.clear();
            temp_arr.push(num.clone());
        }
    }

    res.sort();

    res[res.len() - 1]
}

pub fn longest_consecutive_2(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<i32> = HashSet::from_iter(nums.clone()); 
    let mut longest = 0;

    for n in nums {
        if !num_set.contains(&(n - 1)) {
            let mut length = 0;
            while num_set.contains(&(n + length)) {
                length += 1;
            }
            longest = i32::max(longest, length);
        }
    }

    longest
     
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        let input = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive_2(input), 9);
    }
}
