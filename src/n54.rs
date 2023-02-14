#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

struct Solution;
impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        if matrix.len() == 0 {
            return res;
        }
        let len = matrix.len() * matrix[0].len();

        let (mut row, mut col, mut d_row, mut d_col) = (0, 0, 0 as i32, 1 as i32);
        for _ in 0..len {
            res.push(matrix[row][col]);
            matrix[row][col] = 999;

            if matrix[((row as i32 + d_row) as usize) % matrix.len()]
                [((col as i32 + d_col) as usize) % matrix[0].len()]
                == 999
            {
                let tmp = d_row;
                d_row = d_col;
                d_col = 0 - tmp;
            }

            row = (row as i32 + d_row) as usize;
            col = (col as i32 + d_col) as usize;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::spiral_order(vec![
            [1, 2, 3].to_vec(),
            [4, 5, 6].to_vec(),
            [7, 8, 9].to_vec(),
        ]);
        println!("{:?}", res)
    }
}
