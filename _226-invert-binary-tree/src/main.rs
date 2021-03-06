use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    println!("Hello, world!");
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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
}

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    recursive_invert_tree(&root)
}

fn recursive_invert_tree(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = node {
        let node = &*node.as_ref().borrow();
        let left = recursive_invert_tree(&node.left);
        let right = recursive_invert_tree(&node.right);
        Some(Rc::new(RefCell::new(TreeNode {
            val: node.val,
            left: right,
            right: left,
        })))
    } else {
        return None;
    }
}
