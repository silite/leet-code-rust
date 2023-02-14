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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let root_val = root.clone().unwrap().borrow().val;
        if p.clone().unwrap().borrow().val == root_val
            || q.clone().unwrap().borrow().val == root_val
        {
            return root.clone();
        }

        let left = Solution::lowest_common_ancestor(
            root.clone().unwrap().borrow().left.clone(),
            p.clone(),
            q.clone(),
        );
        let right = Solution::lowest_common_ancestor(
            root.clone().unwrap().borrow().right.clone(),
            p.clone(),
            q.clone(),
        );

        if left.is_none() || right.is_none() {
            return None;
        }

        if left.is_none() {
            return right;
        }
        if right.is_none() {
            return left;
        }

        return root;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
