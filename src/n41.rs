#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        for index in 0..len {
            let index = index as usize;
            let mut target = nums[index];
            while target > 0 && target < len && nums[(target - 1) as usize] != target {
                nums.swap(target as usize - 1, index);
                target = nums[index] - 1;
            }
        }
        println!("{:?}", nums);
        for index in 0..len {
            if nums[index as usize] != index + 1 {
                return index + 1;
            }
        }
        len + 1
    }
}

#[test]
fn test() {
    let res = Solution::first_missing_positive(vec![-1, 4, 2, 1, 9, 10]);
    println!("{}", res);
}
