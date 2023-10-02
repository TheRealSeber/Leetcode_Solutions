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

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_diameter(&root).1
    }
}

fn max_diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    match root {
        None => (0, 0),
        Some(node) => {
            let (l_depth, l_diameter) = max_diameter(&node.borrow().left);
            let (r_depth, r_diameter) = max_diameter(&node.borrow().right);
            (l_depth.max(r_depth) + 1, l_diameter.max(r_diameter).max(l_depth + r_depth))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
