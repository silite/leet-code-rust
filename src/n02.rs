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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: None,
        });
        let mut ptr = &mut dummy;
        while let Some(l_node) = l1.take() {
            if let Some(r_node) = l2.take() {
                let res = l_node.clone().val + r_node.clone().val + carry;

                let mut new_node = Self::get_new_node();
                new_node.val = res % 10;
                ptr.as_mut().next = Some(new_node);
                ptr = ptr.next.as_mut().unwrap();

                carry = res / 10;

                l2 = r_node.next;
            } else {
                l1 = Some(l_node);
                break;
            }
            l1 = l_node.next;
        }
        if l1.is_some() {
            Self::link_remain(l1, &mut ptr, carry);
        } else {
            Self::link_remain(l2, &mut ptr, carry);
        }
        dummy.next
    }

    fn link_remain(mut link: Option<Box<ListNode>>, mut ptr: &mut Box<ListNode>, mut carry: i32) {
        while let Some(node) = link.take() {
            let mut new_node = Self::get_new_node();
            let new_val = node.val + carry;
            new_node.val = new_val % 10;
            carry = new_val / 10;
            ptr.as_mut().next = Some(new_node);
            ptr = ptr.next.as_mut().unwrap();
            link = node.next;
        }
        if carry != 0 {
            let mut new_node = Self::get_new_node();
            new_node.val = carry;
            ptr.as_mut().next = Some(new_node);
        }
    }

    fn get_new_node() -> Box<ListNode> {
        return Box::new(ListNode {
            val: -1,
            next: None,
        });
    }
}

#[test]
fn feature() {
    let res = Solution::add_two_numbers(
        Some(Box::new(ListNode { val: 9, next: None })),
        Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        })),
    );
    println!("{:?}", res)
}
