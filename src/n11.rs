#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        if len <= 1 {
            return 0;
        }
        let mut left = 0;
        let mut right = len - 1;
        let mut max_res = 0;
        while left < right {
            let left_item = height[left];
            let right_item = height[right];
            let res = ((right - left) as i32) * (left_item.min(right_item));
            max_res = max_res.max(res);
            if left_item > right_item {
                right -= 1;
            } else {
                left += 1;
            }
        }
        max_res
    }
}

#[test]
fn test() {
    let ers = Solution::max_area([2, 3, 10, 5, 7, 8, 9].to_vec());
    println!("{}", ers);
}
