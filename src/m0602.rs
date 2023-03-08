#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        let mut res = 0;
        Self::dfs(false, n, k, &mut res, &mut vec![]);
        res
    }

    fn dfs(duplicate: bool, n: i32, k: i32, res: &mut i32, store: &mut Vec<i32>) {
        if store.len() as i32 == n {
            *res += 1;
            // println!("{:?}", store);
            return;
        } else {
            for i in 0..k {
                if duplicate {
                    if let Some(last) = store.last() {
                        if *last == i {
                            continue;
                        }
                    }
                }
                store.push(i);
                let len = store.len();
                let duplicate = if len >= 2 {
                    let a = store.last().unwrap();
                    let b = store.get(len - 2).unwrap();
                    a == b
                } else {
                    false
                };
                Self::dfs(duplicate, n, k, res, store);
                store.pop();
            }
        }
    }
}

#[test]
fn test() {
    let rse = Solution::num_ways(12, 2);
    println!("{}", rse);
}
