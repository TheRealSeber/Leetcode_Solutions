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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match root {
            Some(node) => {
                if node.borrow().val == sub_root.as_ref().unwrap().borrow().val {
                    if Self::is_sametree(Some(node.clone()), sub_root.as_ref()) {
                        return true
                    }
                }
                Self::is_subtree(node.borrow().left.clone(), sub_root.clone()) || Self::is_subtree(node.borrow().right.clone(), sub_root)
            },
            None => false,
        }
    }

    fn is_sametree(l: Option<Rc<RefCell<TreeNode>>>, r: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        match (l, r) {
            (None, None) => true,
            (Some(l_n), Some(r_n)) => {
                l_n.borrow().val == r_n.borrow().val
                    && Self::is_sametree(l_n.borrow().left.clone(), r_n.borrow().left.as_ref())
                    && Self::is_sametree(l_n.borrow().right.clone(), r_n.borrow().right.as_ref())
            }
            _ => false,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
