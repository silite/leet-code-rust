#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let len = coins.len();
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for index in 0..dp.len() {
            for coin in &coins {
                if index as i32 >= *coin {
                    dp[index] = dp[index].min(dp[index - *coin as usize] + 1);
                }
            }
        }
        if dp[amount as usize] == amount + 1 {
            return -1;
        }
        dp[amount as usize]
    }
}

#[test]
fn test() {
    let res = Solution::coin_change(vec![2], 3);
    println!("{}", res);
}
