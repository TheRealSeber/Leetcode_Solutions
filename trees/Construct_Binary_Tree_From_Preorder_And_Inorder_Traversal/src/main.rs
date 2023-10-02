// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
pub struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let mut treenode = TreeNode::new(preorder[0]);
        
        let tn_val_idx = inorder.iter().position(|v| *v == treenode.val).unwrap();

        treenode.left = Self::build_tree(preorder[1..tn_val_idx + 1].to_vec(), inorder[0..tn_val_idx].to_vec());
        treenode.right = Self::build_tree(preorder[tn_val_idx+1..].to_vec(), inorder[tn_val_idx+1..].to_vec());
        
        Some(Rc::new(RefCell::new(treenode)))
    }
}

fn main() {
    println!("Hello, world!");
}
