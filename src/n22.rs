#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        Self::generate(n, String::from(""), &mut vec![], &mut res, n, true);
        res
    }

    fn generate(
        n: i32,
        mut item: String,
        mut queue: &mut Vec<char>,
        res: &mut Vec<String>,
        len: i32,
        flag: bool,
    ) {
        if flag {
            for index in 0..queue.len() {
                let mut tmp = queue.clone();
                for _ in 0..index + 1 {
                    let res = tmp.pop().unwrap();
                    item = format!("{}{}", item, ")")
                }
                if !tmp.is_empty() {
                    Self::generate(n, item.clone(), &mut tmp, res, len, false);
                };
            }
        }

        for index in 0..n {
            let mut tmp_res = String::from("");
            for _ in 0..index + 1 {
                tmp_res += &"(";
                queue.push('(');
            }
            let tmp = &item.clone();
            item = format!("{}{}", item, tmp_res);
            Self::generate(n - index - 1, item.clone(), queue, res, len, true);

            item = tmp.to_owned();
        }

        if item.len() as i32 == len * 2 {
            res.push(item.clone());
        };
    }
}

#[test]
fn test() {
    let res = Solution::generate_parenthesis(3);
    println!("{:?}", res);
}
