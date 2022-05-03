/*
#49
Given an array of meeting time intervals consisting of start
and end times[[s1,e1],[s2,e2],...](si< ei), determine if a person could attend all meetings.

Input:
[[0,30],[5,10],[15,20]]
Output: false

Input:[[7,10],[2,4]]

Output:true
*/

struct Solution;

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        for i in 1..intervals.len() {
            if intervals[i][0] < intervals[i - 1][1] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_attend_meetings() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert_eq!(Solution::can_attend_meetings(intervals), false);
        let intervals = vec![vec![7, 10], vec![2, 4]];
        assert_eq!(Solution::can_attend_meetings(intervals), true);
    }
}
