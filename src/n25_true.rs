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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut ptr = &mut dummy;
        loop {
            let (sub, tail) = Self::reverse_once(head, k);
            head = tail;
            ptr.next = sub;
            while ptr.next.is_some() {
                ptr = ptr.next.as_deref_mut()?;
            }
            if head.is_none() {
                return dummy.next;
            }
        }
    }
    pub fn reverse_once(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut ptr = head.as_deref();
        for _ in 0..k {
            if ptr.is_none() {
                return (head, None);
            }
            ptr = ptr.unwrap().next.as_deref();
        }
        let mut dummy = ListNode::new(-1);
        for _ in 0..k {
            if head.is_none() {
                return (dummy.next, None);
            }
            let mut node = head.take().unwrap(); // node is not none
            head = node.next.take();
            node.next = dummy.next.take();
            dummy.next = Some(node);
        }
        (dummy.next, head)
    }
}
