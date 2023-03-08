#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for (row_index, row_item) in image.iter_mut().enumerate() {
            row_item.reverse();
            for (col_index, col_item) in row_item.iter_mut().enumerate() {
                *col_item = 1 - *col_item;
            }
        }
        image
    }
}
#[test]
fn test() {}
