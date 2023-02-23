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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            Self::bfs(root)
        }
    }

    fn bfs(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        if let Some(node) = root.take() {
            queue.push_back((1, node));
        } else {
            return 0;
        }
        let mut res = 0;
        while queue.len() > 0 {
            let mut queue_len = queue.len();
            let mut layer_list = vec![];
            while queue_len > 0 {
                if let Some((index, node)) = queue.pop_front() {
                    let left = node.clone().borrow_mut().left.take();
                    let right = node.clone().borrow_mut().right.take();
                    if let Some(left) = left {
                        queue.push_back(((2 * index) - 1, left));
                        layer_list.push((2 * index) - 1);
                    }
                    if let Some(right) = right {
                        queue.push_back((2 * index, right));
                        layer_list.push(2 * index);
                    }
                }

                queue_len -= 1;
            }
            if layer_list.is_empty() {
                res = res.max(1);
            } else {
                res = res.max(layer_list.last().unwrap() - layer_list.first().unwrap() + 1);
            }
        }
        res
    }
}

#[test]
fn test() {
    let res = Solution::bfs(get(1, get(2, None, None), get(3, None, None)));
    println!("{}", res);
}
fn get(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}
