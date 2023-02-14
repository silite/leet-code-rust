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
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut views = vec![];
        Self::dfs(root, 0, &mut views);
        views
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, views: &mut Vec<i32>) {
        if let Some(node) = root {
            if views.len() == depth {
                views.push(node.borrow().val);
            } else {
                views[depth] = node.borrow().val;
            }

            Self::dfs(node.borrow().left.clone(), depth + 1, views);
            Self::dfs(node.borrow().right.clone(), depth + 1, views);
        }
    }
}
