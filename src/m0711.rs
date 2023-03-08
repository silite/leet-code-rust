#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let target = nums[(nums.len() / 2)];
        let mut res = 0;
        for item in nums.into_iter() {
            res += (target - item).abs()
        }
        res
    }
}

#[test]
fn test() {
    let res = Solution::min_moves2(vec![1, 0, 0, 8, 6]);
    println!("{}", res);
}
