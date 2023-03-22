#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io::{self, *};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let ll = line.unwrap();
        let numbers: Vec<&str> = ll.split(" ").collect();
        let numbers = numbers[0].trim();
    }
}

#[test]
fn test() {
    main()
}
