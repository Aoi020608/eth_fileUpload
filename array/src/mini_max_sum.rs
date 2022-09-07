#[allow(dead_code)]
pub fn mini_max_sum(arr: &[i32]) -> Vec<i64> {
    let mut max_sum: i64 = 0;
    let mut min_sum: i64 = 0;

    let mut max_num = arr[0];
    for num in arr.iter() {
        if &max_num < num {
            max_num = num.clone();
        }
        max_sum += *num as i64;
    }

    let mut min_num = arr[0];
    for num in arr.iter() {
        if &min_num > num {
            min_num = num.clone();
        }
        min_sum += *num as i64;
    }

    max_sum -= min_num as i64;
    min_sum -= max_num as i64;

    [min_sum, max_sum].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mini_max_sum() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(mini_max_sum(&arr), [16, 24]);
    }

    #[test]
    fn test_mini_max_sum_2() {
        let arr = [396285104, 573261094, 759641832, 819230764, 364801279];
        println!("{:?}", mini_max_sum(&arr));
    }
}
