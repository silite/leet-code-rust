#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
struct Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// 1为成长速度
#[derive(PartialEq, Clone, Debug)]
struct Item(usize, f64);

impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Solution {
    pub fn max_average_ratio(mut classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut b_heap = BinaryHeap::new();
        let len = classes.len();
        for (index, item) in classes.iter().enumerate() {
            let pass = item[0] as f64;
            let total = item[1] as f64;
            let target = Item(index, Self::get_step(pass, total));
            b_heap.push(target);
        }
        println!("{:?}", b_heap);
        let mut extra_students = extra_students;
        while extra_students > 0 {
            if let Some(item) = b_heap.pop() {
                let index = item.0;
                let pass = classes[index][0];
                let total = classes[index][1];
                let new_item = [pass + 1, total + 1];
                classes[index] = new_item.to_vec();

                let new_heap = Item(
                    index,
                    Self::get_step(new_item[0] as f64, new_item[1] as f64),
                );
                b_heap.push(new_heap);
            }
            extra_students -= 1;
        }
        let mut res = 0.0;
        for item in classes {
            let pass = item[0] as f64;
            let total = item[1] as f64;
            res += pass / total;
        }
        res / len as f64
    }

    fn get_step(a: f64, b: f64) -> f64 {
        (a + 1.0) / (b + 1.0) - (a / b)
    }
}

#[test]
fn test() {
    let res = Solution::max_average_ratio(
        [[1, 2].to_vec(), [3, 5].to_vec(), [2, 2].to_vec()].to_vec(),
        2,
    );
    println!("{}", res);
}
