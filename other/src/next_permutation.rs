use std::cmp::Ordering::Less;

struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if let Some(i) = nums.windows(2).rposition(|w| w[0] < w[1]) {
            let (first, elements) = nums[i..].split_first_mut().unwrap();
            elements.reverse();
            println!("elements: {:?}", elements);
            let j = elements
                .binary_search_by(|a| a.cmp(first).then(Less))
                .unwrap_err();
                println!("first: {:?}", first);
            println!("j: {:?}", j);
            nums.swap(i, i + j + 1);
        } else {
            nums.reverse();
        }

        println!("Nums: {:?}", nums);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_next_permutation() {
    //     let mut nums = vec![1, 2, 3];
    //     Solution::next_permutation(&mut nums);
    // }

    // #[test]
    // fn test_next_permutation_01() {
    //     let mut nums = vec![3, 2, 1];
    //     Solution::next_permutation(&mut nums);
    //     // assert_eq!(result, vec![1, 3, 2]);
    // }

    // #[test]
    // fn test_next_permutation_01() {
    //     let mut nums = vec![1, 1, 5];
    //     Solution::next_permutation(&mut nums);
    //     // assert_eq!(result, vec![1, 3, 2]);
    // }

    #[test]
    fn test_next_permutation_01() {
        let mut nums = vec![6, 2, 1, 5, 4, 3, 0];
        // let i = 0;
        // println!("nums array: {:?}", nums[i..]);
        Solution::next_permutation(&mut nums);
        // assert_eq!(result, vec![1, 3, 2]);
    }
}
