// #31

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut ans = 0;

        if len <= 1 {
            if prices[0] < 0 {
                return 0;
            } else {
                return 0;
            }
        }

        for i in 0..len {
            let mut temp_heap = BinaryHeap::new();
            for j in i + 1..len {
                if prices[i] < prices[j] {
                    temp_heap.push(prices[j] - prices[i]);
                }
            }
            if let Some(big_val) = temp_heap.peek() {
                if *big_val > ans {
                    ans = *big_val;
                }
            }
        }

        ans
    }

    #[allow(dead_code)]
    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut ans = prices[0];

        if len <= 1 {
            return 0;
        }

        prices.iter().fold(0, |max_prices, price| {
            ans = ans.min(*price);
            max_prices.max(price - ans)
        })
    }
}

#[test]
fn test_max_profit() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let ans = Solution::max_profit_1(prices);
    assert_eq!(ans, 5);
}

#[test]
fn test_max_profit_2() {
    let prices = vec![7, 6, 4, 3, 1];
    let ans = Solution::max_profit_1(prices);
    assert_eq!(ans, 0);

    let a = [1, 2, 3];

    let sum = a.iter().fold(0, |acc, x| acc + x);

    assert_eq!(sum, 6);
}
