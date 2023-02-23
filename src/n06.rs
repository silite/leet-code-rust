#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut row_res = vec![String::from(""); num_rows as usize];
        let mut acc: i32 = 1;
        let mut res_index: i32 = 0;
        for (index, char) in s.char_indices() {
            row_res[res_index as usize].push_str(char.to_string().as_str());
            if res_index == 0 {
                acc = 1;
            }
            if res_index == num_rows - 1 {
                acc = -1;
            }
            res_index += acc;
        }
        let mut res = String::from("");
        for item in row_res {
            res = res + item.as_str();
        }
        res
    }
}

#[test]
fn test() {
    let res = Solution::convert("AB".to_string(), 1);
    println!("{}", res);
}
