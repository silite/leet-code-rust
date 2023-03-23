#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize]; m as usize];
        for row in 0..m {
            for col in 0..n {
                let row = row as usize;
                let col = col as usize;
                if row * col == 0 {
                    dp[row][col] = 1
                } else {
                    dp[row][col] = dp[row - 1][col] + dp[row][col - 1]
                }
            }
        }
        dp[m as usize - 1][n as usize - 1]
    }
}

#[test]
fn test() {
    let res = Solution::unique_paths(3, 7);
    println!("{}", res);
}
