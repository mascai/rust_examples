// https://leetcode.com/problems/reverse-linked-list/submissions/

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut prev = None;
        let mut next = None;

        while let Some(mut node) = cur {
            next = node.next;
            node.next = prev;

            prev = Some(node);
            cur = next;
        }
        return prev;
    }
}
