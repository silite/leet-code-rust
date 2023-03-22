#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::BinaryHeap;

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

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Some(self.cmp(other))
        None
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: None,
        });
        let mut ptr = &mut dummy;

        let mut heap = BinaryHeap::new();

        for item in lists {
            if let Some(node) = item {
                heap.push(node);
            }
        }
        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(next);
            }
            ptr.next = Some(node);
            ptr = ptr.next.as_mut().unwrap();
        }
        dummy.next
    }
}

#[test]
fn feature() {}
