#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut queue = vec![];
        let mut zero_list = vec![1; s.len()];
        for (index, char) in s.char_indices() {
            if char == '(' {
                queue.push(index);
            } else {
                let pop = queue.pop();
                if pop.is_some() {
                    zero_list[index] = 0;
                    zero_list[pop.unwrap()] = 0;
                }
            }
        }
        let mut max = 0;
        let mut tmp = 0;
        for (index, &item) in zero_list.iter().enumerate() {
            if item == 0 {
                tmp += 1;
            } else {
                max = max.max(tmp);
                tmp = 0;
            }
        }
        max.max(tmp)
    }
}

#[test]
fn test() {
    let res = Solution::longest_valid_parentheses(String::from("(()"));
    println!("{}", res);
}
