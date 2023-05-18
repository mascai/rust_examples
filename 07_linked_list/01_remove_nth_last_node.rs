/*
https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut fake_head = Box::new(ListNode::new(-1));
        fake_head.next = head;

        let mut fast = fake_head.clone(); // copy constructor for Box::new(ListNode)
        let mut slow = fake_head.as_mut(); // &mut ListNode

        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        } 
        slow.next = slow.next.as_mut().unwrap().next.take();
        return fake_head.next;
    }
}
