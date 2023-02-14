#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return Self::mid(&nums, 0, nums.len() - 1, target);
    }
    fn mid(nums: &Vec<i32>, start: usize, end: usize, target: i32) -> i32 {
        if nums[start] == target {
            return start as i32;
        } else if nums[end] == target {
            return end as i32;
        }
        if start >= end {
            return -1;
        }
        let mid = (start + end) / 2;

        // order
        if nums[start] < nums[end] {
            if target > nums[end] || target < nums[start] {
                return -1;
            }
            if nums[mid] >= target {
                return Self::mid(&nums, start, mid, target);
            } else {
                return Self::mid(&nums, mid + 1, end, target);
            }
        } else {
            let mid = (start + end) / 2;
            let left = Self::mid(&nums, start, mid, target);
            let right = Self::mid(&nums, mid + 1, end, target);
            return left.max(right);
        }
    }
}

#[test]
fn test() {
    let res = Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 1);
    println!("{}", res);
}
