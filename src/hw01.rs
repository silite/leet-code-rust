#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io::{self, *};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let ll = line.unwrap();
        let numbers: Vec<&str> = ll.split(" ").collect();
        let mut nums = numbers[0].trim().parse::<i32>().unwrap_or(0);
        if nums == 0 {
            return;
        }
        let mut res = 0;
        while nums > 2 {
            res += nums / 3;
            nums = nums % 3 + nums / 3;
        }
        if nums == 2 {
            res += 1;
        }
        println!("{}\n", res);
    }
}

#[test]
fn test() {
    main()
}
