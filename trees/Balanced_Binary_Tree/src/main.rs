// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
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

pub struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn balanced(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
            match root {
                None => (0, false),
                Some(node) => {
                    let (l_d, l_state) = balanced(node.borrow().left.clone());
                    let (r_d, r_state) = balanced(node.borrow().right.clone());
                    (l_d.max(r_d) + 1, l_state || r_state || (l_d - r_d).abs() > 1)
                }
            }
        }
        !balanced(root).1
    }
}

fn main() {
    println!("Hello, world!");
}
