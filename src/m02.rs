#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut num1: Vec<char> = num1.chars().collect();
        let mut num2: Vec<char> = num2.chars().collect();
        num1.reverse();
        num2.reverse();

        let mut res: Vec<i32> = vec![0; 200];

        for (top_index, top) in num1.iter().enumerate() {
            for (bottom_index, bottom) in num2.iter().enumerate() {
                let top = *top as i32 - 48;
                let bottom = *bottom as i32 - 48;
                res[top_index + bottom_index] = top * bottom + res[top_index + bottom_index];
            }
        }
        for index in 0..res.len() - 1 {
            let item = res[index];
            res[index] = item % 10;
            let carry = item / 10;
            res[index + 1] = res[index + 1] + carry;
        }
        res.reverse();
        let mut tmp = String::from("");
        let mut start = false;
        for item in res {
            if item > 0 {
                start = true;
            }
            if start {
                tmp += item.to_string().as_str();
            }
        }
        if tmp == "".to_string() {
            return "0".to_string();
        }
        tmp
    }
}

#[test]
fn feature() {
    let res = Solution::multiply(String::from("123456789"), String::from("987654321"));
    println!("{}", res);
}
#[test]
fn featur() {
    let res = Solution::multiply(String::from("123"), String::from("456"));
    println!("{}", res);
}

#[test]
fn featu() {
    let mut a = String::from("");
    a.push('b');
    println!("{}", a);
}
