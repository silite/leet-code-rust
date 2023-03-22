#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
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
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = -1;
        Self::dfs(root, &mut vec![max]);
        max
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max: &mut Vec<i32>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let node = root.unwrap();
        let left = Self::dfs(node.clone().borrow().left.clone(), max);
        let right = Self::dfs(node.clone().borrow().right.clone(), max);

        let inner_max = left + right + node.clone().borrow().val;
        max[0] = max[0].max(inner_max);

        let outer_max = node.clone().borrow().val + 0.max(left).max(right);
        return outer_max.max(0);
    }
}

#[test]
fn test() {}
