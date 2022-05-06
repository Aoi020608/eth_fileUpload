// #29

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut temp_vec = Vec::with_capacity(len + 1);
        temp_vec.push(nums[0]);

        if len == 1 {
            return nums[0];
        }

        if nums[1] > nums[0] {
            temp_vec.push(nums[1]);
        } else {
            temp_vec.push(nums[0]);
        }

        for i in 2..len {
            if nums[i] + temp_vec[i - 2] > temp_vec[i - 1] {
                temp_vec.push(nums[i] + temp_vec[i - 2]);
            } else {
                temp_vec.push(temp_vec[i - 1]);
            }
        }

        return temp_vec[len - 1];
    }

    #[allow(dead_code)]
    pub fn rob_1(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(a, b), x| (a.max(b + x), a))
            .0
    }

    #[allow(dead_code)]
    pub fn rob_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_stolen = Vec::with_capacity(n + 1);
        max_stolen.push(nums[0]);

        if n == 1 {
            return nums[0];
        }

        if nums[0] > nums[1] {
            max_stolen.push(nums[0]);
        } else {
            max_stolen.push(nums[1]);
        }

        for index in 2..n {
            if nums[index] + max_stolen[index - 2] > max_stolen[index - 1] {
                max_stolen.push(nums[index] + max_stolen[index - 2]);
            } else {
                max_stolen.push(max_stolen[index - 1]);
            }
        }

        return max_stolen[n - 1];
    }
}

#[test]
fn test_rob() {
    let input = vec![1, 2, 3, 1];
    let ans = Solution::rob_2(input);

    assert_eq!(ans, 4);
}

#[test]
fn test_rob_1() {
    let input = vec![2, 7, 9, 3, 1];
    let ans = Solution::rob_2(input);

    assert_eq!(ans, 12);
}
