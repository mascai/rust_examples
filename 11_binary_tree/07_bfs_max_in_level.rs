/*
https://leetcode.com/problems/find-largest-value-in-each-tree-row/solutions/4201613/rust-solution-bfs-level-order-traversal/?envType=daily-question&envId=2023-10-24
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
use std::collections::VecDeque;


impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);
            while !queue.is_empty() {
                let len = queue.len();
                let mut max = i32::MIN;
                for _ in 0..len {
                    if let Some(root) = queue.pop_front() {
                        let root = root.borrow();
                        if let Some(l) = &root.left {
                            queue.push_back(l.clone());
                        }
                       if let Some(r) = &root.right {
                            queue.push_back(r.clone());
                        }
                        max = max.max(root.val);

                    }
                }
                res.push(max);
            }
        }
        return res;
    }
}
