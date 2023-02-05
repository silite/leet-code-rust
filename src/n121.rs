#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut step_min = i32::MAX;
        let mut res = 0;
        for item in prices {
            step_min = step_min.min(item);
            res = res.max(item - step_min);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::max_profit([7, 6, 4, 3, 1].to_vec());
        println!("{}", res);
    }
}
