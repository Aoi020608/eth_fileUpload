pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ret_vec = Vec::new();
    let mut nums_len = nums.len();

    for i in 0..nums.len() {
        for j in i..nums_len {
            if nums[i] == (target - nums[j]) {
                ret_vec.push(i as i32);
                ret_vec.push(j as i32);
                break;
            }
        }

        nums_len -= 1;
    }

    ret_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let inputs = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(two_sum(inputs, target), [0, 1]);
    }

    #[test]
    fn test_two_sum_1() {
        let inputs = vec![3, 2, 4];
        let target = 6;

        assert_eq!(two_sum(inputs, target), [1, 2]);
    }
}
