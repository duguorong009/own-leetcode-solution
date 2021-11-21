use std::cell::RefCell;
use std::rc::Rc;
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

fn main() {
    println!("Hello, world!");
}

fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let n = inorder.len();
    build(&inorder[0..n], &postorder[0..n])
}

fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let n = inorder.len();
    if n == 0 {
        None
    } else {
        let val = postorder[n - 1];
        let i = inorder.iter().position(|&x| x == val).unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: build(&inorder[0..i], &postorder[0..i]),
            right: build(&inorder[i + 1..n], &postorder[i..n - 1]),
        })))
    }
}
