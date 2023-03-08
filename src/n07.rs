#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x.to_string().chars().collect::<Vec<char>>();
        x.reverse();
        let mut negative = false;
        if *x.last().unwrap() == '-' {
            x.pop();
            negative = true
        }
        let mut x = VecDeque::from(x);
        while let Some(&front) = x.front() {
            if front == '0' {
                x.pop_front();
            }
        }

        0
    }
}
#[test]
fn feature() {
    Solution::reverse(-123);
}
