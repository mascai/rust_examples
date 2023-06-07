/*
https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/description/
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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut res: i32 = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            res <<= 1;
            res |= node.val;
            cur = &node.next;
        }
        return res;
    }
}
