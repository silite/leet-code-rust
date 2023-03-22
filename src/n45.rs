#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![0; len];
        for index in 1..len {
            let mut min_index = 0;
            let mut max_curr = nums[0];
            for j in 0..index {
                // 选的值能jump到
                if index - j > nums[j] as usize {
                    continue;
                }
                min_index = j as i32;
                if index as i32 - min_index <= nums[j] {
                    break;
                }
            }
            dp[index] = dp[min_index as usize] + 1;
        }
        println!("{:?}", dp);
        dp[len - 1]
    }
}

#[test]
fn test() {
    let res = Solution::jump(vec![2, 3, 1, 1, 4]);
    println!("{}", res);
}
