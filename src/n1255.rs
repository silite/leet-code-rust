#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut hash_letter = vec![0; 26];
        for char in letters.into_iter() {
            hash_letter[char as usize - 97] += 1;
        }
        let mut res = vec![];
        Self::dfs(&words, 0, hash_letter, 0, &mut res, &score);
        let mut max = 0;
        res.into_iter().for_each(|x| max = x.max(max));
        max
    }

    fn dfs(
        words: &Vec<String>,
        start: usize,
        mut hash_letter: Vec<i32>,
        mut item_res: i32,
        res: &mut Vec<i32>,
        score: &Vec<i32>,
    ) {
        let len = words.len();

        if start == len {
            res.push(item_res);
            return;
        }

        for index in start..len {
            let mut tmp = hash_letter.clone();
            let recover = hash_letter.clone();
            let mut flag = false;
            let mut item_cnt = 0;
            for item in words[index].chars() {
                let char_index = item as usize - 97;
                tmp[char_index] -= 1;
                item_cnt += score[char_index];
                if tmp[char_index] < 0 {
                    flag = true;
                }
            }
            if !flag {
                item_res += item_cnt;
                hash_letter = tmp;
            }
            Self::dfs(words, index + 1, hash_letter.clone(), item_res, res, score);
            if !flag {
                item_res -= item_cnt;
                hash_letter = recover.clone();
            }
        }
    }
}

#[test]
fn feature() {
    Solution::max_score_words(
        [
            "xxxz".to_string(),
            "ax".to_string(),
            "bx".to_string(),
            "cx".to_string(),
        ]
        .to_vec(),
        ['z', 'a', 'b', 'c', 'x', 'x', 'x'].to_vec(),
        [
            4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
        ]
        .to_vec(),
    );
}
