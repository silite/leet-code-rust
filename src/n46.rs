#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec;

struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut used: Vec<bool> = vec![false; nums.len()];

        let mut tract = vec![];

        Self::back_tract(&mut res, &nums, &mut tract, &mut used);
        res
    }

    fn back_tract(
        res: &mut Vec<Vec<i32>>,
        nums: &Vec<i32>,
        tract: &mut Vec<i32>,
        used: &mut Vec<bool>,
    ) {
        if tract.len() == nums.len() {
            res.push(tract.clone());
        }

        for (index, item) in nums.iter().enumerate() {
            if !used[index] {
                used[index] = true;
                tract.push(*item);

                Self::back_tract(res, &nums, tract, used);

                used[index] = false;
                tract.remove(tract.len() - 1);
            }
        }
    }
}

#[test]
fn test() {
    Solution::permute(vec![1, 2, 3]);
}
