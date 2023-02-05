#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut res = Vec::with_capacity(10000);

        for c in s.chars() {
            match c {
                ' ' => res.append(&mut vec!['%', '2', '0']),
                _ => res.push(c),
            }
        }

        res.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::replace_space("We are happy.".to_string());
        println!("{}", res);
    }
}
