use std::collections::HashMap;

// 两数之和
pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (index, item) in nums.iter().enumerate() {
            if let Some(other_index) = map.get(&(target - item)) {
                if *other_index != index {
                    return vec![*other_index as i32, index as i32];
                }
            }
            map.insert(item, index);
        }
        panic!("无对应数值！")
    }
}
