#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let nums = matrix.len();

        for row_index in 0..nums / 2 {
            for col_index in row_index..(nums - (row_index + 1 / 2)) - 1 {
                let mut cnt = 0;
                let origin = matrix[row_index][col_index];
                let (mut x, mut y) = (row_index, col_index);
                while cnt < 3 {
                    matrix[x][y] = matrix[nums - y - 1][x];
                    cnt += 1;
                    (x, y) = (nums - y - 1, x);
                }
                matrix[x][y] = origin;
            }
        }
    }
}

#[test]
fn test() {
    let mut a = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
    Solution::rotate(&mut a);
    println!("{:?}", a);
}
