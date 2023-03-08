#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;

impl Solution {
    fn pop<T: Eq>(target: &mut Vec<T>, zero: T) {
        while let Some(last) = target.last() {
            if *last == zero {
                target.pop();
            } else {
                break;
            }
        }
    }

    pub fn compare_version(version1: String, version2: String) -> i32 {
        let del_tail_zero = move |str: String| {
            let mut res = str.chars().collect::<Vec<char>>();
            Self::pop(&mut res, '0');
            if res.is_empty() {
                "0".to_string()
            } else {
                res.into_iter().collect::<String>()
            }
        };
        let change = move |str: String| {
            let vec = str.split(".").collect::<Vec<&str>>();
            let mut res = vec
                .into_iter()
                .map(|item| {
                    item.parse::<i32>()
                        .unwrap()
                        .to_string()
                        .parse::<i32>()
                        .unwrap()
                })
                .collect::<Vec<i32>>();
            Self::pop(&mut res, 0);
            res
        };
        let v1 = change(version1);
        let v2 = change(version2);
        let min = v1.len().min(v2.len());
        for index in 0..min {
            let item1 = v1[index];
            let item2 = v2[index];
            if item1 > item2 {
                return 1;
            }
            if item1 < item2 {
                return -1;
            }
        }
        if v1.len() > v2.len() {
            return 1;
        }
        if v1.len() < v2.len() {
            return -1;
        }
        0
    }
}

#[test]
fn test() {
    let res = Solution::compare_version(String::from("1.1"), String::from("1.10"));
    println!("{}", res);
}
