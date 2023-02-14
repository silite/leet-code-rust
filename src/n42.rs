#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let len = height.len();

        let mut max_left: Vec<i32> = vec![];
        let mut max_right: Vec<i32> = vec![0; len];

        let mut left = 0;
        let mut right = len - 1;

        while left < len {
            if left == 0 {
                max_left.push(height[0]);
            } else {
                max_left.push(height[left].max(max_left[left - 1]));
            }
            left += 1;
        }

        while right > 0 {
            if right == len - 1 {
                max_right[len - 1] = *height.last().unwrap();
            } else {
                max_right[right] = height[right].max(max_right[right + 1]);
            }
            right -= 1;
        }

        for (index, item) in height.iter().enumerate() {
            res += (max_left[index].min(max_right[index]) - item).max(0);
        }
        res
    }
}

#[test]
fn test() {
    let res = Solution::trap([4, 2, 0, 3, 2, 5].to_vec());
    println!("{}", res);
}
