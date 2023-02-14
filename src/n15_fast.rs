impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..n - 2 {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            let mut left = i + 1;
            let mut right = n - 1;
            while left < right {
                if left > i + 1 && nums[left - 1] == nums[left] {
                    left += 1;
                    continue;
                }
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        res
    }
}
