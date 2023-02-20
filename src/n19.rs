#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{ops::Deref, rc::Rc};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut t = &mut head.clone();
        let mut dummy = Box::new(ListNode::new(-1));
        dummy.next = head;
        let mut h = &mut dummy;
        while let Some(node) = t {
            if n == 0 {
                h = h.next.as_mut().unwrap();
            } else {
                n -= 1;
            }
            t = &mut t.as_mut().unwrap().next;
        }

        h.next = h.next.as_mut().unwrap().next.take();
        dummy.next
    }
}

#[test]
fn test() {
    let res = Solution::remove_nth_from_end(
        Some(get_node(
            1,
            Some(get_node(2, Some(get_node(3, Some(get_node(4, None)))))),
        )),
        2,
    );
    println!("{:?}", res);
}
fn get_node(val: i32, next: Option<Box<ListNode>>) -> Box<ListNode> {
    Box::new(ListNode { val, next })
}
