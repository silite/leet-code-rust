#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut left = 0_usize;
        let mut right = nums.len() - 1;
        let mut hash = HashMap::new();

        while left < right {
            let res: f32 = (nums[left] as f32 + nums[right] as f32) / 2.0;
            let target = hash.entry(res.to_string()).or_insert(1);
            left += 1;
            right -= 1;
        }
        hash.len() as i32
    }
}

#[test]
fn test() {}
