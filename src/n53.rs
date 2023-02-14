#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut res = nums[0];
        for item in nums {
            if item > 0 {
                sum += item;
            } else {
                sum = item;
            }
            res = res.max(sum);
        }
        res
    }
}

#[test]
fn test() {}
