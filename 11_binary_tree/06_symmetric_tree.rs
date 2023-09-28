/*
https://leetcode.com/problems/symmetric-tree/description/
*/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn check(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match(l, r) {
            (None, None) => true,
            (None, Some(n)) | (Some(n), None) => false,
            (Some(l), Some(r)) => {
                return l.borrow().val == r.borrow().val &&
                       Self::check(l.borrow().left.clone(), r.borrow().right.clone()) &&
                       Self::check(l.borrow().right.clone(), r.borrow().left.clone());
            }
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(root) => Self::check(
                root.borrow().left.clone(),
                root.borrow().right.clone()
            ),
            None => true
        }
    }
}
