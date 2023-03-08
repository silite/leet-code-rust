#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut res = HashMap::new();
        let mut last_time = 0;
        for (index, char) in keys_pressed.chars().enumerate() {
            let target = res.entry(char).or_insert((0, index));
            target.0 = target.0.max(release_times[index] - last_time);
            last_time = release_times[index];
        }
        let mut a = '\0';
        let mut max = 0;
        let mut res = res.into_iter().collect::<Vec<(char, (i32, usize))>>();
        res.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        for (char, (val, _)) in res.into_iter() {
            if val > max {
                max = val;
                a = char;
            }
        }
        a
    }
}

#[test]
fn test() {
    let res = Solution::slowest_key([9, 29, 49, 50].to_vec(), "cbcd".to_string());
    println!("{}", res);
}
