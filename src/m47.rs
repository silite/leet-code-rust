#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        Self::dfs(&grid, &mut res, &mut 0, (0, 0));
        res
    }

    fn dfs(grid: &Vec<Vec<i32>>, res: &mut i32, item_res: &mut i32, index: (usize, usize)) {
        let row_len = grid.len();
        let col_len = grid[0].len();
        if index.0 == row_len || index.1 == col_len {
            *res = *res.max(&mut *item_res);
            return;
        }
        for i in 0..2 {
            match i {
                0 => {
                    if index.0 < row_len {
                        let target_val = grid[index.0][index.1];
                        *item_res += target_val;
                        if index.0 + 1 == row_len {}
                        Self::dfs(
                            grid,
                            res,
                            item_res,
                            if index.0 + 1 == row_len {
                                (index.0, index.1 + 1)
                            } else {
                                (index.0 + 1, index.1)
                            },
                        );
                        *item_res -= target_val;
                    }
                }
                1 => {
                    if index.1 < col_len {
                        let target_val = grid[index.0][index.1];
                        *item_res += target_val;
                        Self::dfs(
                            grid,
                            res,
                            item_res,
                            if index.1 + 1 == col_len {
                                (index.0 + 1, index.1)
                            } else {
                                (index.0, index.1 + 1)
                            },
                        );
                        *item_res -= target_val;
                    }
                }
                _ => {}
            }
        }
    }
}
#[test]
fn test() {
    let res = Solution::max_value(
        [
            [7, 1, 3, 5, 8, 9, 9, 2, 1, 9, 0, 8, 3, 1, 6, 6, 9, 5].to_vec(),
            [9, 5, 9, 4, 0, 4, 8, 8, 9, 5, 7, 3, 6, 6, 6, 9, 1, 6].to_vec(),
            [8, 2, 9, 1, 3, 1, 9, 7, 2, 5, 3, 1, 2, 4, 8, 2, 8, 8].to_vec(),
            [6, 7, 9, 8, 4, 8, 3, 0, 4, 0, 9, 6, 6, 0, 0, 5, 1, 4].to_vec(),
            [7, 1, 3, 1, 8, 8, 3, 1, 2, 1, 5, 0, 2, 1, 9, 1, 1, 4].to_vec(),
            [9, 5, 4, 3, 5, 6, 1, 3, 6, 4, 9, 7, 0, 8, 0, 3, 9, 9].to_vec(),
            [1, 4, 2, 5, 8, 7, 7, 0, 0, 7, 1, 2, 1, 2, 7, 7, 7, 4].to_vec(),
            [3, 9, 7, 9, 5, 8, 9, 5, 6, 9, 8, 8, 0, 1, 4, 2, 8, 2].to_vec(),
            [1, 5, 2, 2, 2, 5, 6, 3, 9, 3, 1, 7, 9, 6, 8, 6, 8, 3].to_vec(),
            [5, 7, 8, 3, 8, 8, 3, 9, 9, 8, 1, 9, 2, 5, 4, 7, 7, 7].to_vec(),
            [2, 3, 2, 4, 8, 5, 1, 7, 2, 9, 5, 2, 4, 2, 9, 2, 8, 7].to_vec(),
            [0, 1, 6, 1, 1, 0, 0, 6, 5, 4, 3, 4, 3, 7, 9, 6, 1, 9].to_vec(),
        ]
        .to_vec(),
    );
    println!("{}", res);
}
