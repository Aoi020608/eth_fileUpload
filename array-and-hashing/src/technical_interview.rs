pub fn question_1() -> Vec<i32> {
    let mut array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let array_len = array.len();

    for index in 0..array_len / 2 {
        let front_el = array[index];
        let back_el = array[array_len - index - 1];

        array[index] = back_el;
        array[array_len - index - 1] = front_el;
    }

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_question_1() {
        let mut array = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(array, question_1());
    }
}
