use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

struct Solution;

impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>, 
        low: i32, 
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            if node.val > high {
                return Self::trim_bst(left, low, high);
            }
            if node.val < low {
                return Self::trim_bst(right, low, high);
            }
            node.left = Self::trim_bst(left, low, high);
            node.right = Self::trim_bst(right, low, high);
            root
        } else {
            None
        }
    }
}