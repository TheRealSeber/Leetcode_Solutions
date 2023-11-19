use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

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
            right: None,
        }
    }

    pub fn pre_vec(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match root {
            Some(node) => {
                result.push(node.borrow().val);
                Self::pre_vec(&node.as_ref().borrow().left, result);
                Self::pre_vec(&node.as_ref().borrow().right, result);
            }
            None => return,
        }
    }

    pub fn in_vec(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match root {
            Some(node) => {
                Self::in_vec(&node.as_ref().borrow().left, result);
                result.push(node.borrow().val);
                Self::in_vec(&node.as_ref().borrow().right, result);
            }
            None => return,
        }
    }

    pub fn post_vec(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match root {
            Some(node) => {
                Self::post_vec(&node.as_ref().borrow().left, result);
                Self::post_vec(&node.as_ref().borrow().right, result);
                result.push(node.borrow().val);
            }
            None => return,
        }
    }

    pub fn construct_prein(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let mut treenode = TreeNode::new(preorder[0]);

        let val_idx = inorder
            .iter()
            .position(|v| *v == treenode.val)
            .expect("No value found!");

        treenode.left = Self::construct_prein(&preorder[1..val_idx + 1], &inorder[0..val_idx]);
        treenode.right = Self::construct_prein(&preorder[val_idx + 1..], &inorder[val_idx + 1..]);

        Some(Rc::new(RefCell::new(treenode)))
    }

    pub fn construct_inpost(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }

        let mut treenode = TreeNode::new(postorder[postorder.len() - 1]);

        let val_idx = inorder
            .iter()
            .position(|v| *v == treenode.val)
            .expect("No value found!");

        treenode.left = Self::construct_inpost(&inorder[..val_idx], &postorder[..val_idx]);
        treenode.right = Self::construct_inpost(
            &inorder[val_idx + 1..],
            &postorder[val_idx..postorder.len() - 1],
        );

        Some(Rc::new(RefCell::new(treenode)))
    }

    pub fn construct_prepost(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match preorder.len() {
            0 => return None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(preorder[0])))),
            _ => {
                let mut treenode = TreeNode::new(preorder[0]);

                let val_idx = postorder
                    .iter()
                    .position(|v| *v == preorder[1])
                    .expect("No value found!");

                treenode.left =
                    Self::construct_prepost(&preorder[1..val_idx + 2], &postorder[..val_idx + 1]);
                treenode.right = Self::construct_prepost(
                    &preorder[val_idx + 2..],
                    &postorder[val_idx + 1..postorder.len() - 1],
                );

                Some(Rc::new(RefCell::new(treenode)))
            }
        }
    }
}
#[derive(Default)]
pub struct Solution {
    result: Vec<Vec<i32>>,
}

impl Solution {
    pub fn output(&self) {
        let mut out = io::BufWriter::new(io::stdout());

        for vecs in &self.result {
            for value in vecs {
                write!(out, "{} ", value).expect("Failed to write to file");
            }
            writeln!(out, "").expect("Failed to write to file");
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer: String = String::with_capacity(10);
    io::stdin().read_line(&mut buffer).unwrap();    
    let data_sets_num = buffer
        .trim()
        .parse::<i32>()
        .expect("Error parsing the number of data sets!");

    if data_sets_num < 1 || data_sets_num > 2000000000 {
        panic!("Incorrect ammout of datasets!")
    }

    let mut solution = Solution::default();

    for _ in 0..data_sets_num {
        let mut traversal_type_buffer = String::new();
        io::stdin()
            .read_line(&mut traversal_type_buffer)
            .expect("Error loading traversal type!");
        let traversal_types: Vec<&str> = traversal_type_buffer.split_whitespace().collect();

        let mut values_1_buffer = String::new();
        io::stdin()
            .read_line(&mut values_1_buffer)
            .expect(&format!("Error loading values of {}", traversal_types[0]));
        let values_vec_1: Vec<i32> = values_1_buffer
            .split_whitespace()
            .map(|f| f.parse::<i32>().expect("Error parsing the values to i32!"))
            .collect();

        let mut values_2_buffer = String::new();
        io::stdin()
            .read_line(&mut values_2_buffer)
            .expect(&format!("Error loading values of {}", traversal_types[1]));
        let values_vec_2: Vec<i32> = values_2_buffer
            .split_whitespace()
            .map(|f| f.parse::<i32>().expect("Error parsing the values to i32!"))
            .collect();

        let mut res: Vec<i32> = Vec::with_capacity(values_vec_1.len());
        match (traversal_types[0], traversal_types[1]) {
            ("PREORDER", "INORDER") => {
                let tree = TreeNode::construct_prein(&values_vec_1, &values_vec_2);
                TreeNode::post_vec(&tree, &mut res);
            }
            ("INORDER", "POSTORDER") => {
                let tree = TreeNode::construct_inpost(&values_vec_1, &values_vec_2);
                TreeNode::pre_vec(&tree, &mut res);
            }
            ("PREORDER", "POSTORDER") => {
                let tree = TreeNode::construct_prepost(&values_vec_1, &values_vec_2);
                TreeNode::in_vec(&tree, &mut res);
            }
            _ => panic!("Wrong traversal type!"),
        }
        solution.result.push(res);
    }
    solution.output();
    Ok(())
}
