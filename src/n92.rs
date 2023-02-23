#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
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
impl Solution {
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        mut right: i32,
    ) -> Option<Box<ListNode>> {
        if left >= right {
            return head;
        }

        let mut dummy = Box::new(ListNode {
            val: -1,
            next: None,
        });

        let mut ptr_reverse = &mut dummy;
        let mut ptr_list = &mut head;

        let mut start = left;
        while start > 0 {
            let node = ListNode::new(ptr_list.clone().unwrap().val);
            ptr_reverse.next = Some(Box::new(node));
            ptr_list = &mut ptr_list.as_mut().unwrap().next;
            ptr_reverse = ptr_reverse.next.as_mut().unwrap();
            start -= 1;
        }

        while right - left > 0 {
            right -= 1;
        }

        println!("{:?}", dummy.next);
        None
    }
}

#[test]
fn test() {
    Solution::reverse_between(Some(Box::new(ListNode::new(1))), 1, 2);
}
