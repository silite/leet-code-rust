#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash = HashMap::new();
        for (index, item) in strs.iter().enumerate() {
            let mut item_chars = item.chars().collect::<Vec<char>>();
            item_chars.sort_unstable();
            let item_chars = item_chars.iter().collect::<String>();
            let target = hash.entry(item_chars).or_insert(vec![]);
            target.push(String::from(item));
        }
        hash.into_iter().map(|(_, str_list)| str_list).collect()
    }
}

#[test]
fn test() {
    let res = Solution::group_anagrams(
        [
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]
        .to_vec(),
    );
    println!("{:?}", res);
}
