#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let cnt = if n < 2 { 2 } else { n as usize };
        let mut dp = vec![0; cnt + 1];
        dp[1] = 1;
        dp[2] = 2;

        for i in 3..n + 1 {
            let i = i as usize;
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
}

#[test]
fn test() {
    let res = Solution::climb_stairs(4);
    println!("{}", res);
}
