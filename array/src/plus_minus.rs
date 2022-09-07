#[allow(dead_code)]
pub fn plus_minus(arr: &[i32]) -> Vec<f32> {
    let len = arr.len();
    let mut pos = 0.0;
    let mut neg = 0.0;
    let mut zeros = 0.0;

    let zero = 0;

    for num in arr.iter() {
        if num < &zero {
            neg += 1.0;
        } else if num == &zero {
            zeros += 1.0;
        } else {
            pos += 1.0;
        }
    }

    let num_pos = pos / len.clone() as f32;
    let num_neg = neg / len.clone() as f32;
    let num_zeros = zeros / len.clone() as f32;

    [num_pos as f32, num_neg as f32, num_zeros as f32].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_minus() {
        let arr = [1, 1, 0, -1, -1];
        assert_eq!(plus_minus(&arr), [0.400000, 0.400000, 0.200000]);
    }
}
