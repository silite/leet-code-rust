#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
struct Solution;
impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let n = n as usize;
        let res = s[n..].to_string() + &s[..n].to_string();
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::reverse_left_words("abcdefg".to_string(), 2);
        println!("{}", res);
    }
}
