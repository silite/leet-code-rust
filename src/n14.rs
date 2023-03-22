#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::from(&strs[0]);
        for item in &strs {
            if item.len() < res.len() {
                res = res[..item.len()].to_string();
            }
            for (index, char) in item.char_indices() {
                if let Some(res_char) = res.chars().nth(index) {
                    if res_char != char {
                        res = res[..index].to_string();
                    }
                }
            }
        }
        res.into()
    }
}

#[test]
fn test() {
    let res = Solution::longest_common_prefix(vec![
        "reflower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
    println!("{}", res);
}
