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

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(l_node), Some(r_node)) => match l_node.borrow().val == r_node.borrow().val {
                true => {
                    Self::is_same_tree(l_node.borrow().left.clone(), r_node.borrow().left.clone())
                        && Self::is_same_tree(
                            l_node.borrow().right.clone(),
                            r_node.borrow().right.clone(),
                        )
                }
                false => false,
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}
