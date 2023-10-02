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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: i32, res: &mut Vec<i32>) {
            match node {
                Some(n) => {
                    let nb = n.borrow();
                    if depth > res.len() as i32 {
                        res.push(nb.val);
                    }
                    dfs(&nb.right, depth + 1, res);
                    dfs(&nb.left, depth + 1, res);
                },
                None => (),
            }
        }
        match root {
            Some(_) => {
                let mut res = vec![];
                let res_ptr = &mut res;
                dfs(&root, 1, res_ptr);
                res
            },
            None => vec![],
        }
    }
}

fn main() {
    println!("Hello, world!");
}
