#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let (mut row_index, mut col_index) = (0, 0);
        while row_index < grid.len() {
            while col_index < grid[0].len() {
                let item = grid[row_index][col_index];
                if item == '1' {
                    Solution::dfs(&mut grid, [row_index, col_index], item);
                    res += 1;
                }
                col_index += 1;
            }
            row_index += 1;
            col_index = 0;
        }
        res
    }
    fn dfs(grid: &mut Vec<Vec<char>>, start: [usize; 2], target: char) {
        let mut fake_queue = vec![start];
        loop {
            if fake_queue.len() == 0 {
                break;
            }
            let [row_index, col_index] = fake_queue.pop().unwrap();

            if grid[row_index][col_index] == '1' {
                grid[row_index][col_index] = '2';
                let row_index = row_index as i32;
                let col_index = col_index as i32;
                if Solution::judge(&grid, row_index - 1, col_index) {
                    fake_queue.push([(row_index - 1) as usize, col_index as usize]);
                }
                if Solution::judge(&grid, row_index + 1, col_index) {
                    fake_queue.push([(row_index + 1) as usize, col_index as usize]);
                }
                if Solution::judge(&grid, row_index, col_index - 1) {
                    fake_queue.push([row_index as usize, (col_index - 1) as usize]);
                }
                if Solution::judge(&grid, row_index, col_index + 1) {
                    fake_queue.push([row_index as usize, (col_index + 1) as usize]);
                }
            }
        }
    }

    fn judge(grid: &Vec<Vec<char>>, row_index: i32, col_index: i32) -> bool {
        if 0 <= row_index
            && 0 <= col_index
            && (row_index as usize) < grid.len()
            && (col_index as usize) < grid[row_index as usize].len()
        {
            return grid[row_index as usize][col_index as usize] == '1';
        }
        false
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test() {
        let res = Solution::num_islands(vec![
            ['1', '1', '0', '0', '0'].to_vec(),
            ['1', '1', '0', '0', '0'].to_vec(),
            ['0', '0', '1', '0', '0'].to_vec(),
            ['0', '0', '0', '1', '1'].to_vec(),
        ]);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_error() {
        let res = Solution::num_islands(vec![vec!['1']]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_error2() {
        let res = Solution::num_islands(vec![
            ['1', '1', '1'].to_vec(),
            ['0', '1', '0'].to_vec(),
            ['1', '1', '1'].to_vec(),
        ]);
        assert_eq!(res, 1);
    }
}
