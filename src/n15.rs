#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let len = nums.len();
        if len < 3 {
            return res;
        }
        nums.sort();
        for index in 0..len - 2 {
            let item = nums[index];

            if index > 0 && item == nums[index - 1] {
                continue;
            }
            let mut left = index + 1;
            let mut right = len - 1;
            while left < right {
                if left > index + 1 && nums[left - 1] == nums[left] {
                    left += 1;
                    continue;
                }

                let cnt = item + nums[left] + nums[right];
                if cnt == 0 {
                    res.push(vec![nums[index], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                } else if cnt < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let a = vec![-1, 0, 1, 2, -1, -4];
        let res = Solution::three_sum(a);
        println!("{:?}", res);
    }
}
