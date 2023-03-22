#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let len_1 = text1.len();
        let len_2 = text2.len();

        let mut res = vec![vec![0; len_2 + 1]; len_1 + 1];
        for (i, x) in text1.chars().enumerate() {
            for (j, y) in text2.chars().enumerate() {
                if x == y {
                    res[i + 1][j + 1] = res[i][j] + 1;
                } else {
                    res[i + 1][j + 1] = res[i][j + 1].max(res[i + 1][j]);
                }
            }
        }
        return res[len_1][len_2];
    }
}

#[test]
fn test() {
    let res = Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string());
    println!("{}", res);
}
