#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = vec![intervals[0].clone()];
        for item in intervals {
            let start = item[0];
            let end = item[1];
            let last = res.last().unwrap();
            let len = res.len();
            if start <= last[1] {
                res[len - 1] = vec![last[0], last[1].max(end)];
            } else {
                res.push(vec![start, end]);
            }
        }
        res
    }
}

#[test]
fn test() {}
