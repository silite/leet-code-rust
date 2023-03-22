#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = vec![vec![0; n as usize]; n as usize];

        let mut return_res = vec![];

        Self::dfs(&mut res, n, 0, 0, &mut return_res);

        return_res
    }

    fn dfs(
        res: &mut Vec<Vec<i32>>,
        n: i32,
        curr: i32,
        queue: i32,
        return_res: &mut Vec<Vec<String>>,
    ) {
        if queue >= n {
            return_res.push(Self::get_res(res));
            return;
        }

        for row in curr..n {
            for col in 0..n {
                res[row as usize][col as usize] = 1;
                if !Self::judge(res, row, col) {
                    res[row as usize][col as usize] = 0;
                    continue;
                }
                Self::dfs(res, n, row + 1, queue + 1, return_res);
                // Self::change_status(res, row, col, false);
                res[row as usize][col as usize] = 0;
            }
        }
    }

    fn get_res(res: &mut Vec<Vec<i32>>) -> Vec<String> {
        let mut return_res = vec![];
        let len = res.len();
        for row in 0..len {
            let mut item_res = String::from("");
            for col in 0..len {
                let curr = res[row][col];
                if curr == 1 {
                    item_res += "Q";
                } else {
                    item_res += ".";
                }
            }
            return_res.push(item_res);
        }
        return_res
    }

    fn judge(res: &mut Vec<Vec<i32>>, row: i32, col: i32) -> bool {
        let row = row as usize;
        let col = col as usize;
        let n = res.len();
        for index in 1..n {
            if res[row][(index + col) % n] == 1 {
                return false;
            }
        }
        for index in 1..n {
            if res[(index + row) % n][col] == 1 {
                return false;
            }
        }
        for index in 1..row + 1 {
            if col as i32 - index as i32 >= 0 {
                if res[row - index][col - index] == 1 {
                    return false;
                }
            }
            if col + index < n {
                if res[row - index][col + index] == 1 {
                    return false;
                }
            }
        }
        for index in 1..(n - row) {
            if col as i32 - index as i32 >= 0 {
                if res[row + index][col - index] == 1 {
                    return false;
                }
            }
            if col + index < n {
                if res[row + index][col + index] == 1 {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let res = Solution::solve_n_queens(9);
    println!("{:?}", res)
}
