#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0;
        let len = nums.len();
        if len < 2 {
            return;
        }
        let mut right = len - 1;
        while left <= right {
            if nums[left] > nums[right] {
                nums.swap(left, right);
            } else if nums[left] == nums[right] {
                let mut tmp = left;
                while tmp <= nums.len() - 2 {
                    tmp += 1;
                    if nums[tmp] != nums[tmp - 1] {
                        break;
                    }
                }
                if tmp >= right {
                    break;
                }
                if nums[tmp] > nums[right] {
                    nums.swap(tmp, right);
                } else {
                    nums.swap(tmp, left);
                }
            }
            if nums[left] == 0 {
                left += 1;
            }
            if nums[right] == 2 {
                right -= 1;
            }
        }
    }
}

#[test]
fn test() {
    let mut nums = [1, 1, 2, 0, 1, 1, 1, 2].to_vec();
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
}
