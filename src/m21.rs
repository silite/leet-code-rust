#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
type uszie_yuan = (usize, usize);
impl Solution {
    pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
        let mut infected_area = vec![];
        for row_index in 0..is_infected.len() {
            for col_index in 0..is_infected[0].len() {
                if is_infected[row_index][col_index] == 1 {
                    Self::dfs(&mut is_infected, &mut infected_area, row_index, col_index);
                }
            }
        }

        0
    }

    fn dfs(
        is_infected: &mut Vec<Vec<i32>>,
        res: &mut Vec<uszie_yuan>,
        row_index: usize,
        col_index: usize,
    ) {
        let mut queue = vec![(row_index, col_index)];
        res.push(queue[0]);
        while !queue.is_empty() {
            let target = queue.pop().unwrap();
            let mut sur = Self::get_sur(target.0, target.1, &is_infected);
            sur = sur
                .into_iter()
                .filter(|(x, y)| is_infected[*x][*y] == 1)
                .collect();
            res.append(sur.clone().as_mut());
            queue.append(sur.clone().as_mut());
            sur.push(target);
            for (x, y) in &sur {
                is_infected[*x][*y] = -1;
            }
        }
    }

    fn get_sur(row_index: usize, col_index: usize, is_infected: &Vec<Vec<i32>>) -> Vec<uszie_yuan> {
        let (top, bottom, left, right) = ((-1, 0), (1, 0), (0, -1), (0, 1));
        let mut res = vec![];
        let row_index = row_index as i32;
        let col_index = col_index as i32;

        let get_new_point = |(dx, dy)| (((row_index + dx) as usize), (col_index + dy) as usize);

        if row_index > 0 {
            res.push(get_new_point(top));
        }
        if row_index < is_infected.len() as i32 - 1 {
            res.push(get_new_point(bottom));
        }
        if col_index > 0 {
            res.push(get_new_point(left));
        }
        if col_index < is_infected[0].len() as i32 - 1 {
            res.push(get_new_point(right))
        }
        res
    }

    fn move_infect(is_infected: &mut Vec<Vec<i32>>) {}
}

#[test]
fn test() {
    Solution::contain_virus(vec![
        [0, 1, 0, 0, 0, 0, 0, 1].to_vec(),
        [0, 1, 0, 0, 0, 0, 0, 1].to_vec(),
        [0, 0, 0, 0, 0, 0, 0, 1].to_vec(),
        [0, 0, 0, 0, 0, 0, 0, 0].to_vec(),
    ]);
}
