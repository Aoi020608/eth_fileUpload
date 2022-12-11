pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut clo_nums = nums.clone();

    let mut prefix = 1;
    let mut postfix = 1;

    for (_, num) in nums.into_iter().enumerate() {
        res.push(prefix);
        prefix = prefix * num;
    }

    let mut len_res = res.len();
    clo_nums.reverse();

    for num in clo_nums {
        res[len_res - 1] *= postfix;
        postfix = postfix * num.clone();
        len_res -= 1;
    }

    res
}

#[cfg(test)]
mod tests {
    // use std::vec;

    use super::*;

    #[test]
    fn test_product_except_itself() {
        let input = vec![1, 2, 3, 4];

        assert_eq!(product_except_self(input), vec![24, 12, 8, 6]);
    }
}
