#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;

use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct Queue {
    value: Vec<Rc<RefCell<TreeNode>>>,
    curr: usize,
    flag: bool,
}
impl Queue {
    fn new() -> Self {
        Self {
            value: Vec::new(),
            curr: 0,
            // true left    false right
            flag: false,
        }
    }

    fn pop(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
        let res;
        if self.curr < self.value.len() {
            res = Some(self.value[self.curr].clone());
        } else {
            res = None;
        }
        self.curr += 1;
        if self.curr == self.value.len() {
            self.curr = 0;
            self.value.clear();
        }
        res
    }

    fn push(&mut self, item: Rc<RefCell<TreeNode>>) {
        self.value.push(item);
    }
}
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = Queue::new();
        if let Some(node) = root {
            queue.push(node);
        }
        let mut res: Vec<Vec<i32>> = vec![];

        Solution::insert_level(&mut queue, &mut res);

        res
    }

    fn insert_level(queue: &mut Queue, res: &mut Vec<Vec<i32>>) {
        let mut len = queue.value.len();
        let mut res_item: Vec<i32> = vec![];
        let mut pending_push = vec![];
        while len > 0 {
            let item = queue.pop().unwrap();

            let mut left = item.borrow().left.clone();
            let mut right = item.borrow().right.clone();
            if queue.flag {
                let tmp = right;
                right = left;
                left = tmp;
            }

            if left.is_some() {
                pending_push.push(left);
            }
            if right.is_some() {
                pending_push.push(right);
            }

            res_item.push(item.borrow().val);

            len -= 1;
        }

        pending_push.reverse();
        pending_push
            .into_iter()
            .for_each(|x| queue.push(x.unwrap()));
        queue.flag = !queue.flag;
        if res_item.len() > 0 {
            res.push(res_item)
        };
        if queue.value.len() != 0 {
            Solution::insert_level(queue, res);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let res = Solution::zigzag_level_order(tree);
        println!("{:?}", res);
    }
}
