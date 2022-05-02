/*
#39
Implement pow(x, n), which calculates x raised to the power n (i.e., xn).

Input: x = 2.00000, n = 10
Output: 1024.00000

*/

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        x.powf(n as f64)
    }

    pub fn my_pow_1(x: f64, n: i32) -> f64 {
        if 0 > n {
            Solution::_my_pow(1f64 / x, -n)
        } else {
            Solution::_my_pow(x, n)
        }
    }

    fn _my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1f64;
        }

        let aux = Solution::_my_pow(x, n / 2);
        if n % 2 == 0 {
            aux * aux
        } else {
            aux * aux * x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_pow_1() {
        let x = 2.00000;
        let n = 10;
        let res = Solution::my_pow_1(x, n);
        assert_eq!(res, 1024.00000);
    }

    #[test]
    fn test_my_pow_2() {
        let x = 2.10000;
        let n = 3;
        let res = Solution::my_pow_1(x, n);
        assert_eq!(res, 9.261000000000001);
    }

    #[test]
    fn test_my_pow_3() {
        let x = 2.00000;
        let n = -2;
        let res = Solution::my_pow_1(x, n);
        assert_eq!(res, 0.25);
    }
}
