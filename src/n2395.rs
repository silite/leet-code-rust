#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut hash = HashMap::new();
        for i in 0..nums.len() - 1 {
            let sum = nums[i as usize] + nums[i as usize + 1];
            hash.entry(sum).and_modify(|res| *res += 1).or_insert(1);
            if let Some(target) = hash.get(&sum) {
                if target > &1 {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let res = Solution::find_subarrays(vec![4, 2, 4]);
    println!("{}", res);
}
