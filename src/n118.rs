#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        for index in 1..num_rows + 1 {
            let mut item = vec![1; index as usize];
            for inner_index in 1..index - 1 {
                let inner_index = inner_index as usize;
                item[inner_index] =
                    res.last().unwrap()[inner_index - 1] + res.last().unwrap()[inner_index];
            }
            res.push(item);
        }
        res
    }
}

#[test]
fn test() {
    let res = Solution::generate(5);
    println!("{:?}", res);
}
