
use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut q = VecDeque::new();
        let k = k as usize;
        for i in 0..k - 1 {
            while !q.is_empty() && nums[*q.back().unwrap()] <= nums[i] {
                q.pop_back();
            }
            q.push_back(i);
        }
        for i in k - 1..nums.len() {
            while !q.is_empty() && nums[*q.back().unwrap()] <= nums[i] {
                q.pop_back();
            }
            q.push_back(i);
            while let Some(&v) = q.front() {
                if v + k > i {
                    res.push(nums[v]);
                    break;
                } else {
                    q.pop_front();
                }
            }
        }
        res
    }
}
///如果a和b出现在一个窗口中，b在a的右边且b比a大，那么此后a永远不会成为某个窗口的最大值了。
/// 维护一个单调递减的队列，队列头部的值就是当前窗口的最大值。
