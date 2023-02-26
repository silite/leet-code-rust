#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let len = s.len();
        let even = len / 4;
        let mut hash = HashMap::new();
        for char in s.chars() {
            let ptr = hash.entry(char).or_insert(0);
            *ptr += 1;
        }
        let mut res = 0;
        println!("{:?}{}", hash, even);
        for (_, value) in hash.into_iter() {
            res += (value as i32 - even as i32).max(0)
        }
        res
    }
}

#[test]
fn test() {
    let res = Solution::balanced_string("WWEQERQWQWWRWWERQWEQ".to_string());
    println!("{}", res);
}
