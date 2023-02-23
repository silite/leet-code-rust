#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::back(&candidates, target, &mut res, &mut vec![], 0);
        res
    }

    fn back(
        candidates: &Vec<i32>,
        target: i32,
        res: &mut Vec<Vec<i32>>,
        item: &mut Vec<i32>,
        start: usize,
    ) {
        let sum: i32 = item.iter().sum();
        if sum == target {
            res.push(item.clone());
            return;
        }
        if sum > target {
            return;
        }

        for index in start..candidates.len() {
            item.push(candidates[index]);
            Self::back(candidates, target, res, item, index);
            item.pop();
        }
    }
}

#[test]
fn test() {
    let res = Solution::combination_sum([2, 3, 5].to_vec(), 8);
    println!("{:?}", res)
}
