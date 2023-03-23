#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (len1, len2, len3) = (s1.len(), s2.len(), s3.len());
        if len1 + len2 != len3 {
            return false;
        }
        let mut res = vec![];
        Self::dfs(&s1, &s2, &s3, &mut res);
        res.into_iter().any(|x| x)
    }

    fn dfs(s1: &String, s2: &String, s3: &String, res: &mut Vec<bool>) {
        if s3.is_empty() {
            res.push(true);
            return;
        }
        let a = s1.chars().nth(0).unwrap_or_default();
        let b = s2.chars().nth(0).unwrap_or_default();
        let char = s3.chars().nth(0).unwrap_or_default();
        if a == b && a == char {
            Self::dfs(&s1[1..].to_string(), s2, &s3[1..].to_string(), res);
            Self::dfs(s1, &s2[1..].to_string(), &s3[1..].to_string(), res);
        } else if char == a {
            Self::dfs(&s1[1..].to_string(), s2, &s3[1..].to_string(), res);
        } else if char == b {
            Self::dfs(s1, &s2[1..].to_string(), &s3[1..].to_string(), res);
        } else {
            res.push(false);
            return;
        }
    }
}

#[test]
fn test() {
    let res = Solution::is_interleave(
        "aabcc".to_string(),
        "dbbca".to_string(),
        "aadbbcbcac".to_string(),
    );
    println!("{}", res);
}
