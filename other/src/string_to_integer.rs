struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut trimmed = s.trim().to_string();
        if trimmed == "".to_string() {
            return 0;
        }

        let start = trimmed.chars().next().unwrap();
        let mut signed: i64 = 1;
        let mut total: i64 = 0;

        if start == '+' {
            signed = 1;
        } else if start == '-' {
            signed = -1;
        } else if start.is_digit(10) {
            total += i64::from(start.to_digit(10).unwrap());
        } else {
            return 0;
        }

        trimmed = trimmed.chars().skip(1).collect();

        for i in trimmed.chars() {
            if i.is_digit(10) {
                total *= 10;
                total += signed * i64::from(i.to_digit(10).unwrap());
            } else {
                break;
            }

            if total > i64::from(i32::max_value()) {
                return i32::max_value();
            } else if total < i64::from(i32::min_value()) {
                return i32::min_value();
            }
        }

        return total as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_atoi() {
        let s = "42";
        let result = Solution::my_atoi(s.to_string());
        assert_eq!(result, 42);
    }

    #[test]
    fn test_my_atoi_02() {
        let s = "-42";
        let result = Solution::my_atoi(s.to_string());
        assert_eq!(result, -42);
    }

    #[test]
    fn test_my_atoi_03() {
        let s = "4193 with words";
        let result = Solution::my_atoi(s.to_string());
        assert_eq!(result, 4193);
    }
}
