#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res = vec![];
        if s.len() > 12 || s.len() < 4 {
            return res;
        }

        Self::dfs(&s, &mut vec![], 0, &mut res);
        res
    }

    fn dfs(s: &String, item: &mut Vec<String>, start: usize, res: &mut Vec<String>) {
        if item.len() == 4 && start == s.len() {
            res.push(item.join("."));
            return;
        }
        if item.len() == 4 {
            return;
        }

        for len in 1..4 {
            if start + len - 1 >= s.len() {
                return;
            }
            if len != 1 && Self::get_string(&s, start, start + 1) == "0" {
                return;
            }

            let sub_str = Self::get_string(&s, start, start + len);
            if len == 3 && sub_str.parse::<i32>().unwrap() > 255 {
                return;
            }

            item.push(sub_str);
            Self::dfs(&s, item, start + len, res);
            item.pop();
        }
    }

    fn get_string(s: &String, start_index: usize, end_index: usize) -> String {
        let s_chars = s.chars().collect::<Vec<char>>();
        return s_chars[start_index..end_index]
            .to_vec()
            .iter()
            .collect::<String>();
    }
}
#[test]
fn test() {
    let res = Solution::restore_ip_addresses(String::from("25525511135"));
    println!("{:?}", res)
}
