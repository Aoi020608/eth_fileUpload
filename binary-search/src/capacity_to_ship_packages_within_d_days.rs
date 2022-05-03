/*
#38
A conveyor belt has packages that must be shipped from one port to another within days days.

The ith package on the conveyor belt has a weight of weights[i].
Each day, we load the ship with packages on the conveyor belt (in the order given by weights).
We may not load more weight than the maximum weight capacity of the ship.

Return the least weight capacity of the ship that will result in all the packages on the conveyor belt being shipped within days days.

Input: weights = [1,2,3,4,5,6,7,8,9,10], days = 5
Output: 15
Explanation: A ship capacity of 15 is the minimum to ship all the packages in 5 days like this:
1st day: 1, 2, 3, 4, 5
2nd day: 6, 7
3rd day: 8
4th day: 9
5th day: 10

Note that the cargo must be shipped in the order given, so using a ship of capacity 14 and
splitting the packages into parts like (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) is not allowed.

*/

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut left = weights.iter().max().unwrap().clone();
        let mut right: i32 = weights.iter().sum();

        while left < right {
            let mid = (left + right) / 2;

            if Self::can_ship(&mid, &weights, days) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return right;
    }

    fn can_ship(mid: &i32, weights: &Vec<i32>, days: i32) -> bool {
        let mut cur_weight = 0;
        let mut days_taken = 1;

        for weight in weights.iter() {
            cur_weight += weight;

            if cur_weight > *mid {
                days_taken += 1;
                cur_weight = *weight;
            }
        }

        return days_taken <= days;
    }
}

// T: log(N) * O(N)
// S: O(1)

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ship_within_days() {
        let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let days = 5;
        let result = Solution::ship_within_days(weights, days);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_ship_within_days_1() {
        let weights = vec![3, 2, 2, 4, 1, 4];
        let days = 3;
        let result = Solution::ship_within_days(weights, days);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_ship_within_days_2() {
        let weights = vec![1, 2, 3, 1, 1];
        let days = 4;
        let result = Solution::ship_within_days(weights, days);
        assert_eq!(result, 3);
    }
}
