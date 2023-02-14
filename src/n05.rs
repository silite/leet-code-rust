#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let char_vec: Vec<char> = s.chars().collect();
        let mut max_len = 0;
        let mut max_start_index = 0;
        for (index, char) in s.char_indices() {
            let mut tmp_max_len = 1;
            let mut tmp_max_start_index = index;
            let mut left = index;
            let mut right = index;
            while left > 0 && char_vec[left - 1] == char {
                left -= 1;
                tmp_max_len += 1;
                tmp_max_start_index -= 1;
            }
            while right + 1 < len && char_vec[right + 1] == char {
                right += 1;
                tmp_max_len += 1;
            }
            while left > 0 && right < len - 1 {
                left -= 1;
                right += 1;
                if char_vec[left] == char_vec[right] {
                    tmp_max_len += 2;
                    tmp_max_start_index -= 1;
                } else {
                    break;
                }
            }

            if tmp_max_len > max_len {
                max_len = tmp_max_len;
                max_start_index = tmp_max_start_index;
            }
        }
        return char_vec[max_start_index..max_start_index + max_len]
            .into_iter()
            .collect();
    }
}

#[test]
fn test() {
    let res = Solution::longest_palindrome(String::from("cccc"));
    println!("{}", res);
}
