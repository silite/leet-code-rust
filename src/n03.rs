#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut repeat_index = 0;
        let mut bulk = HashMap::new();
        for (index, char) in s.char_indices() {
            if let Some(fetch) = bulk.get(&char) {
                res = res.max(index - repeat_index);
                repeat_index = repeat_index.max(*fetch + 1)
            }
            bulk.insert(char, index);
        }
        if res < s.len() - repeat_index {
            return s.len() as i32 - repeat_index as i32;
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("auqwervio")),
            9
        );

        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );

        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );

        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
    }

    #[test]
    fn test_error() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("dvdf")),
            3
        );
    }

    #[test]
    fn test_error2() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abba")),
            2
        );
    }
}
