#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0];
        let mut max = 0;
        for (index, item) in nums.iter().enumerate() {
            if index > 0 {
                let mut tmp_dp_max = 0;
                for dp_index in 0..index {
                    if nums[dp_index] < *item && tmp_dp_max < dp[dp_index] {
                        tmp_dp_max = dp[dp_index];
                    }
                }
                if max <= tmp_dp_max {
                    max = tmp_dp_max + 1;
                }
                dp.push(tmp_dp_max + 1);
            } else {
                dp[0] = 1;
                max = 1;
            }
        }
        max
    }
}

#[test]
fn test() {
    let res = Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]);
    println!("{}", res);
}
