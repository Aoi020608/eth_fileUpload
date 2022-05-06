// #32

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).fold(
            0,
            |acc, x| {
                if x[0] < x[1] {
                    x[1] - x[0] + acc
                } else {
                    acc
                }
            },
        )
    }

    #[allow(dead_code)]
    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        prices.iter().zip(prices[1..].iter()).fold(
            0,
            |acc, (a, b)| {
                if a < b {
                    b - a + acc
                } else {
                    acc
                }
            },
        )
    }
}

#[test]
fn test_max_profit() {
    let input = vec![7, 1, 5, 3, 6, 4];
    let ans = Solution::max_profit_1(input);
    assert_eq!(ans, 7);
}

#[test]
fn test_max_profit_2() {
    let input = vec![1, 2, 3, 4, 5];
    let ans = Solution::max_profit_1(input);
    assert_eq!(ans, 4);
}

#[test]
fn test_max_profit_3() {
    let input = vec![7, 6, 4, 3, 1];
    let ans = Solution::max_profit_1(input);
    assert_eq!(ans, 0);
}
