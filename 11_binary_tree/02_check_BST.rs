/*
https://leetcode.com/problems/validate-binary-search-tree/description/
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
    pub fn check(root: Option<Rc<RefCell<TreeNode>>>, mut mmin: i64, mut mmax: i64) -> bool {
        if root.is_none() {
            return true;
        }
        let node = root.unwrap();
        if node.borrow().val as i64 <= mmin || node.borrow().val as i64 >= mmax {
            return false;
        }
        return Self::check(node.borrow().left.clone(), mmin, node.borrow().val as i64) &&
               Self::check(node.borrow().right.clone(), node.borrow().val as i64, mmax);
    }
 
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut mmax = std::i64::MAX;
        let mut mmin = std::i64::MIN;
        return Self::check(root, mmin, mmax);   
    }
}
