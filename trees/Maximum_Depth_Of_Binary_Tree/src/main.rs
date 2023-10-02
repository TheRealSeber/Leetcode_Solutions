// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

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
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}

trait MaxDepth {
    fn node_depth(root: &Rc<RefCell<TreeNode>>, max_depth: i32) -> i32;
}

impl MaxDepth for Solution {
    fn node_depth(root: &Rc<RefCell<TreeNode>>, max_depth: i32) -> i32 {
        if root.borrow().left.is_some() && root.borrow().right.is_some() {
            Self::node_depth(root.borrow().left.as_ref().unwrap(), max_depth + 1).max(
                Self::node_depth(root.borrow().right.as_ref().unwrap(), max_depth + 1),
            )
        } else if root.borrow().left.is_some() {
            Self::node_depth(root.borrow().left.as_ref().unwrap(), max_depth + 1)
        } else if root.borrow().right.is_some() {
            Self::node_depth(root.borrow().right.as_ref().unwrap(), max_depth + 1)
        } else {
            max_depth
        }
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = &root {
            Self::node_depth(root, 1)
        } else {
            0
        }
    }
}

fn main() {
    println!("Hello, world!");
}
