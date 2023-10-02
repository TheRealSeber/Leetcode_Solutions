// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> TreeNode {
        Self {
            val,
            left: None,
            right: None,
        }
    }
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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, p_num: i32, q_num: i32, res_ptr: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = node {

                let nb = node.borrow();

                *res_ptr = Some(Rc::new(RefCell::new(TreeNode::new(nb.val))));

                if nb.val > p_num && nb.val > q_num {
                    dfs(&nb.left, p_num, q_num, res_ptr);
                }

                if nb.val < p_num && nb.val < q_num {
                    dfs(&nb.right, p_num, q_num, res_ptr);
                }
            }
        }
        let mut res = None;
        let res_ptr = &mut res;
        dfs(&root, p.unwrap().borrow().val, q.unwrap().borrow().val, res_ptr);
        res
    }
}

fn main() {
    println!("Hello world!");
}