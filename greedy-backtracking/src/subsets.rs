/*
Given an integer array nums of unique elements, return all possible subsets (the power set).

The solution set must not contain duplicate subsets. Return the solution in any order.

Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

*/

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![vec![]];

        let mut tmp;

        for num in nums {
            let mut sub = vec![];
            for item in &res {
                tmp = item.clone();
                tmp.push(num);
                sub.push(tmp);
            }
            res.append(&mut sub);
        }

        res
    }

    pub fn subsets_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut temp: Vec<i32> = vec![];
        let mut res: Vec<Vec<i32>> = vec![vec![]];

        let mut stack: Vec<(Vec<i32>, usize)> = vec![(nums, 0)];
        while let Some((s_nums, start_index)) = stack.pop() {
            let n = s_nums.to_vec();
            for i in start_index..len {
                temp.push(n[i]);
                stack.push((n.to_vec(), i + 1));
            }

            res.push(temp.to_vec());
            temp.pop();
        }

        println!("Result: {:?}", res);

        res
    }
}

// 2 ^ n
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_subsets() {
        let nums = vec![1, 2, 3];
        let result = Solution::subsets_1(nums);
        assert_eq!(
            result,
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn test_subsets_1() {
        let nums = vec![0];
        let result = Solution::subsets_1(nums);
        assert_eq!(result, vec![vec![], vec![0],]);
    }
}
