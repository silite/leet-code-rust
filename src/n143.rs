#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {}
}

#[test]
fn feature() {}
