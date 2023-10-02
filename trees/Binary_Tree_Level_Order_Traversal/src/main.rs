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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: i32, res: &mut Vec<Vec<i32>>) {
            match node {
                Some(n) => {
                    let nb = n.borrow();
                    match depth >= res.len() as i32 {
                        true => {
                            res.push(vec![nb.val]);
                        },
                        false => {
                            res[depth as usize].push(nb.val);
                        },
                    }
                    dfs(&n.borrow().left, depth + 1, res);
                    dfs(&n.borrow().right, depth + 1, res)
                },
                None => (),
            }
        }
        let mut res = vec![vec![]];
        let res_ref = &mut res;
        dfs(&root, 0, res_ref);
        match res[0].len() {
            0 => vec![],
            _ => res
        }
    }
}

fn main() {
    println!("Hello, world!");
}
