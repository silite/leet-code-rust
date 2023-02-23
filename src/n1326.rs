#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let len = ranges.len();
        let mut range_list = vec![];
        for (index, &item) in ranges.iter().enumerate() {
            let index = index as i32;
            let left = (index - item).max(0);
            let right = (index + item).min(n);

            range_list.push([left, right]);
        }
        range_list.sort_by(|a, b| a[0].cmp(&b[0]));
        println!("{:?}", range_list);
        let mut dp = vec![i32::MAX; n as usize + 1];
        dp[0] = 0;
        for item in range_list {
            let start = item[0] as usize;
            let end = item[1] as usize;

            if dp[start] == i32::MAX {
                return -1;
            }

            for index in start..end + 1 {
                dp[index] = dp[index].min(dp[start] + 1);
            }
        }
        dp[n as usize]
    }
}

#[test]
fn test() {
    let res = Solution::min_taps(
        70,
        [
            2, 4, 3, 1, 3, 0, 0, 0, 3, 0, 0, 4, 4, 4, 3, 5, 5, 1, 3, 4, 1, 2, 5, 2, 4, 4, 4, 1, 2,
            1, 1, 1, 3, 1, 0, 4, 5, 5, 1, 4, 2, 3, 5, 4, 3, 0, 3, 1, 0, 5, 5, 4, 2, 4, 4, 2, 2, 1,
            1, 5, 2, 4, 1, 5, 5, 2, 2, 4, 2, 4, 0,
        ]
        .to_vec(),
    );
    println!("{}", res);
}
