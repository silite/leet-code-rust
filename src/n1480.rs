struct Solution;
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len < 2 {
            return nums;
        }
        for index in 1..len {
            nums[index] = nums[index] + nums[index - 1];
        }
        nums
    }
}

#[test]
fn feature() {}
