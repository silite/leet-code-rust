#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ops::Deref;

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

struct Change {
    head: Option<Box<ListNode>>,
    tail: Option<Box<ListNode>>,
    origin: i32,
    curr: i32,
}

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut ptr = &mut dummy;
        loop {
            let (new_head, tail) = Solution::reverse(head, k);
            ptr.next = new_head;
            while ptr.next.is_some() {
                ptr = ptr.next.as_deref_mut()?;
            }
            head = tail;
            if head.is_none() {
                return dummy.next;
            }
        }
    }

    fn reverse(
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
            // 取走，断next
            let mut node = head.take().unwrap();
            // head指向下一节点，为了循环
            head = node.next.take();
            // 断走dummy的next，前插node
            node.next = dummy.next.take();
            // 接上
            dummy.next = Some(node);
        }
        (dummy.next, head)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        println!("{:?}", head);
        let res = Solution::reverse_k_group(head, 2);
        println!("{:?}", res);
    }
}
