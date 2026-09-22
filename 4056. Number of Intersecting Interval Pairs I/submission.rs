use std::cmp::{max, min};

impl Solution {
    pub fn count_intersecting_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        let mut hit = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                let x = &intervals[i];
                let y = &intervals[j];
                if max(x[0], y[0]) <= min(x[1], y[1]) {
                    hit += 1;
                }
            }
        }
        hit
    }
}
