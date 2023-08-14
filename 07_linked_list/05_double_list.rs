/*
https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/submissions/
*/

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut prev = None;
        let mut next = None;

        while let Some(mut node) = curr {
            next = node.next;
            node.next = prev;
            prev = Some(node);
            curr = next;
        }
        return prev;
    }

    pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rev_head = Self::reverse(head);
        let mut curr = rev_head.as_mut().unwrap();
        let mut carry = 0;

        loop {
            let val = carry + curr.val * 2;
            curr.val = val % 10;
            carry = val / 10;

            if curr.next.is_none() {
                break;
            }
            curr = curr.next.as_mut().unwrap();
        }

        if carry > 0 {
            let new_node = Box::new(ListNode::new(carry));
            curr.next = Some(new_node);
            curr = curr.next.as_mut().unwrap();
        }
        return Self::reverse(rev_head);
    }
}
