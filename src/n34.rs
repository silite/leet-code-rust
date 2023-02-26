#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let index = Self::divide(&nums, 0, nums.len() - 1, target);
        if let Some(index) = index {
            let mut left = index;
            let mut right = index;
            while left > 0 && nums[left - 1] == target {
                left -= 1;
            }
            while right < nums.len() - 1 && nums[right + 1] == target {
                right += 1;
            }
            return vec![left as i32, right as i32];
        } else {
            vec![-1, -1]
        }
    }
    fn divide(nums: &Vec<i32>, start: usize, end: usize, target: i32) -> Option<usize> {
        let index = (start + end) / 2;
        if nums[index] == target {
            return Some(index);
        }
        if start >= end {
            return None;
        }
        if nums[index] > target {
            return if index > 0 {
                Self::divide(nums, start, index - 1, target)
            } else {
                None
            };
        } else {
            return if index < nums.len() - 1 {
                Self::divide(nums, index + 1, end, target)
            } else {
                None
            };
        }
    }
}

#[test]
fn test() {
    let res = Solution::search_range([1, 2, 3].to_vec(), 1);
    println!("{:?}", res);
}
