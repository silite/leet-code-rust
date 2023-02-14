// https://leetcode.cn/company/bytedance/problemset/

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for i in 0..k {
            heap.push(Reverse(nums[i as usize]));
        }
        for i in k as usize..nums.len() {
            if let Some(x) = heap.peek() {
                if nums[i] > x.0 {
                    heap.pop();
                    heap.push(Reverse(nums[i as usize]));
                }
            }
        }
        heap.pop().unwrap().0
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::find_kth_largest([6, 5, 4, 3, 2, 1].to_vec(), 2);
        println!("{}", res);
    }
}
