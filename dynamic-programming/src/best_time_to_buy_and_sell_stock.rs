/*
#31
You are given an array prices where prices[i] is the price of a given stock on the ith day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

Input: prices = [7,1,5,3,6,4]
Output: 5
Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
*/

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
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

    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut ans = prices[0];

        if len <= 1 {
            if prices[0] < 0 {
                return 0;
            } else {
                return 0;
            }
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

    // the sum of all of the elements of the array
    let sum = a.iter().fold(0, |acc, x| acc + x);

    assert_eq!(sum, 6);
}

// fold
