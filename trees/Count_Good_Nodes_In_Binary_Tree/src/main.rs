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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, last_max: i32, count: &mut i32) {
            match node {
                Some(n) => {
                    let nb = n.borrow();

                    if last_max <= nb.val {
                        *count += 1;
                    }
                    dfs(&nb.right, nb.val.max(last_max), count);
                    dfs(&nb.left, nb.val.max(last_max), count);
                },
                None => (),
            }
        }
        let max = root.clone().unwrap().borrow().val;
        let mut res = 0;
        let res_ptr = &mut res;
        dfs(&root, max, res_ptr);
        res
    }
}

fn main() {
    println!("Hello, world!");
}
