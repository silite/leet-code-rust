#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut queue = vec![];
        for (index, char) in s.char_indices() {
            if char == '{' || char == '[' || char == '(' {
                queue.push(char);
            } else {
                let res = Self::pop(&mut queue);
                match char {
                    '}' => {
                        if res != '{' {
                            return false;
                        }
                    }
                    ')' => {
                        if res != '(' {
                            return false;
                        }
                    }
                    ']' => {
                        if res != '[' {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
        }
        queue.len() == 0
    }

    fn pop(queue: &mut Vec<char>) -> char {
        let pop_item = queue.pop();
        if let Some(item) = pop_item {
            item
        } else {
            '\0'
        }
    }
}

#[test]
fn test() {
    let res = Solution::is_valid("[".to_string());
    println!("{}", res);
}
