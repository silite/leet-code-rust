#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![nums[0]];
        for (index, item) in nums.iter().enumerate() {
            if item > dp.last().unwrap() {
                dp.push(*item);
            } else {
                let dp_index = dp.binary_search(item).unwrap_or_else(|x| x);
                dp[dp_index] = *item;
            }
        }
        dp.len() as i32
    }
}

#[test]
fn test() {
    let res = Solution::length_of_lis(vec![0, 1, 6, 3, 2, 3]);
    println!("{}", res);
}
