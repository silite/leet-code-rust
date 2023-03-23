#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = vec![];
        let mut key_hash = HashMap::new();
        key_hash.insert('2', vec!['a', 'b', 'c']);
        key_hash.insert('3', vec!['d', 'e', 'f']);
        key_hash.insert('4', vec!['g', 'h', 'i']);
        key_hash.insert('5', vec!['j', 'k', 'l']);
        key_hash.insert('6', vec!['m', 'n', 'o']);
        key_hash.insert('7', vec!['p', 'q', 'r', 's']);
        key_hash.insert('8', vec!['t', 'u', 'v']);
        key_hash.insert('9', vec!['w', 'x', 'y', 'z']);
        if digits.is_empty() {
            return res;
        }

        Self::dfs(&key_hash, &digits, 0, &String::from(""), &mut res);

        res
    }

    fn dfs(
        key_hash: &HashMap<char, Vec<char>>,
        digits: &String,
        start: i32,
        item_res: &String,
        res: &mut Vec<String>,
    ) {
        let len = digits.len() as i32;
        if item_res.len() == len as usize {
            res.push(String::from(item_res));
            return;
        }
        for index in start..len {
            let curr = key_hash
                .get(&digits.chars().nth(index as usize).unwrap())
                .unwrap();
            for item in curr.iter() {
                Self::dfs(
                    key_hash,
                    digits,
                    index + 1,
                    &format!("{}{}", item_res, item),
                    res,
                );
            }
        }
    }
}

#[test]
fn test() {
    let res = Solution::letter_combinations("".to_string());
    println!("{:?}", res)
}
