#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let mut hash_suits = HashMap::new();
        for item in suits {
            let target = hash_suits.entry(item).or_insert(0);
            *target += 1;
        }
        if hash_suits.len() == 1 {
            return String::from("Flush");
        }
        let mut hash_ranks = HashMap::new();
        for item in ranks {
            let target = hash_ranks.entry(item).or_insert(0);
            *target += 1;
        }
        let mut max_dui = 0;
        for (key, &value) in &hash_ranks {
            max_dui = max_dui.max(value);
        }
        if max_dui >= 3 {
            return String::from("Three of a Kind");
        } else if max_dui >= 2 {
            return String::from("Pair");
        }
        if hash_ranks.len() == 5 {
            return String::from("High Card");
        }
        String::from("")
    }
}

#[test]
fn test() {}
