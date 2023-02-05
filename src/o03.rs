#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in nums {
            let res = map.entry(i).or_insert(0);
            *res += 1;
            if *res != 1 {
                return i;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::find_repeat_number([2, 3, 1, 0, 2, 5, 3].to_vec());
        println!("{}", res);
    }
}
