#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut cnt = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    if dp[i] == dp[j] + 1 {
                        cnt[i] += cnt[j];
                    } else if dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1;
                        cnt[i] = cnt[j];
                    }
                }
            }
        }
        let max = dp.iter().max().unwrap();
        let mut ans = 0;
        for i in 0..n {
            if dp[i] == *max {
                ans += cnt[i];
            }
        }
        ans
    }
}

#[test]
fn test() {
    let res = Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]);
    println!("{}", res);
}
