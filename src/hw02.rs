#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    collections::HashSet,
    io::{self, *},
};

fn main() {
    let stdin = io::stdin();
    let mut hash = HashSet::new();
    let mut flag = false;
    let mut cnt = 0;
    for line in stdin.lock().lines() {
        let ll = line.unwrap();
        let numbers: Vec<&str> = ll.split(" ").collect();
        let input = numbers[0].trim().parse::<i32>().unwrap_or(0);

        if !flag {
            flag = true;
            cnt = input;
        } else {
            cnt -= 1;
            hash.insert(input);
        }

        if cnt == 0 {
            let mut res: Vec<i32> = hash.clone().into_iter().collect();
            res.sort_unstable();
            for item in res.into_iter() {
                println!("{}", item);
            }
        }
    }
}

#[test]
fn test() {
    main()
}
