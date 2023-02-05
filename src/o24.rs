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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;
        let mut head = head;
        while let Some(mut var) = head.take() {
            let node_next = var.next.take();
            let mut cp_node = var;
            cp_node.next = res;
            res = Some(cp_node);
            head = node_next;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::reverse_list(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })));

        println!("{:?}", res);
    }
}
