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
type OptionNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn is_valid_bst(root: OptionNode) -> bool {
        Self::is_valid(&root, i32::MIN as i64 - 1, i32::MAX as i64 + 1)
    }
    
    fn is_valid(node: &OptionNode, gt: i64, lt: i64) -> bool {
        match node.as_ref() {
            None => true,
            Some(n) => {
                let b = n.borrow();
                (b.val as i64) > gt && (b.val as i64) < lt
                    && Self::is_valid(&b.left, gt, b.val as i64)
                    && Self::is_valid(&b.right, b.val as i64, lt)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
