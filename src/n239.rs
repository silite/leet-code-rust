#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::BinaryHeap;

struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut queue = BinaryHeap::new();
        let mut start_down = k;
        for (index, &item) in nums.iter().enumerate() {
            if start_down > 1 {
                queue.push((item, index));
                start_down -= 1;
            } else {
                let out_index = index as i32 - k;
                if out_index < 0 {
                    queue.push((item, index));
                    res.push(queue.peek().unwrap().clone().0);
                } else {
                    let last_max = res.last().unwrap();
                    if *last_max == nums[out_index as usize] {
                        queue.push((item, index));
                        while let Some(top) = queue.peek() {
                            if top.1 as i32 >= index as i32 - k + 1 {
                                res.push(top.0);
                                break;
                            }
                            queue.pop();
                        }
                    } else {
                        queue.push((item, index));
                        res.push(*last_max.max(&item));
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let res = Solution::max_sliding_window([-7, -8, 7, 5, 7, 1, 6, 0].to_vec(), 4);
    println!("{:?}", res);
}
