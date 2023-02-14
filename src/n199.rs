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
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }
        let mut bfs_list = vec![root.clone().unwrap()];
        Self::bfs(&root, &mut bfs_list, &mut res);
        res
    }

    fn bfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        bfs_list: &mut Vec<Rc<RefCell<TreeNode>>>,
        res: &mut Vec<i32>,
    ) {
        if let Some(last) = bfs_list.clone().last() {
            res.push(last.borrow().val);
        } else {
            return;
        }
        let mut tmp_res = vec![];
        for node in bfs_list.clone() {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            if let Some(left) = left {
                tmp_res.push(left);
            }
            if let Some(right) = right {
                tmp_res.push(right);
            }
        }
        bfs_list.clear();
        bfs_list.append(&mut tmp_res);
        Self::bfs(root, bfs_list, res);
    }
}

#[test]
fn feature() {
    let res = Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    }))));
    println!("{:?}", res)
}
