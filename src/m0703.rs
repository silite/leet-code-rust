#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
        coins.sort_unstable();
        let mut count = 1;
        for v in coins {
            if count >= v {
                count += v;
            } else {
                break;
            }
        }

        count
    }
}
#[test]
fn test() {
    let res = Solution::get_maximum_consecutive(vec![
        1, 89, 8, 1, 47, 34, 99, 1, 1, 1, 55, 89, 1, 52, 36, 1, 62, 1, 1, 1, 4, 27, 1, 45, 1, 1,
        48, 1, 94, 1, 63,
    ]);
    println!("{}", res);
}
