#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut flag = 0;
        for (index, item) in arr.iter().enumerate() {
            if index + 1 < arr.len() {
                if *item == arr[index + 1] {
                    return false;
                }

                if *item < arr[index + 1] {
                    if flag == 0 {
                        flag += 1;
                    }
                    if flag != 1 {
                        return false;
                    }
                }
                if *item > arr[index + 1] {
                    if flag == 1 {
                        flag += 1;
                    }
                    if flag != 2 {
                        return false;
                    }
                }
            }
        }
        flag == 2
    }
}

#[test]
fn feature() {
    let res = Solution::valid_mountain_array(vec![0, 1, 2, 1, 2]);
    println!("{}", res);
}
