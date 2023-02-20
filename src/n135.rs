#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let len = ratings.len();
        let mut candy_list_left = vec![-1; len];
        candy_list_left[0] = 1;
        for index in 1..len {
            if ratings[index] > ratings[index - 1] {
                candy_list_left[index] = candy_list_left[index - 1] + 1;
            } else {
                candy_list_left[index] = 1;
            }
        }

        let mut candy_list_right = vec![-1; len];
        candy_list_right[len - 1] = 1;
        for fake_index in 1..len {
            let index = len - fake_index - 1;
            if ratings[index] > ratings[index + 1] {
                candy_list_right[index] = candy_list_right[index + 1] + 1;
            } else {
                candy_list_right[index] = 1;
            }
        }
        let mut res = 0;
        for index in 0..len {
            res += candy_list_left[index].max(candy_list_right[index]);
        }
        res
    }
}

#[test]
fn test() {
    let res = Solution::candy([1, 2, 87, 87, 87, 2, 1].to_vec());
    println!("{}", res);
}
