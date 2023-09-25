/*
https://leetcode.com/submissions/detail/1059050057/
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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match(p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let pp = p.borrow();
                let qq = q.borrow();
                return pp.val == qq.val &&
                       Self::is_same_tree(pp.left.clone(), qq.left.clone()) &&
                       Self::is_same_tree(pp.right.clone(), qq.right.clone());
            },
            _ => false
        }
             
    }
}
