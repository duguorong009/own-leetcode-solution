use std::rc::Rc;
use std::cell::RefCell;

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

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

macro_rules! tree {
    ($v: expr, $left: expr, $right: expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $v,
            left: $left,
            right: $right,
        })))
    };
    ($v: expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $v,
            left: None, 
            right: None,
        })))
    }
}

struct Solution;

impl Solution {
    pub fn kth_smallest(root: TreeLink, k: i32) -> i32 {
        let mut traversed: Vec<i32> = vec![];
        Self::in_order_traverse(&root, &mut traversed);

        traversed[k as usize - 1]
    }

    fn in_order_traverse(node: &TreeLink, arr: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            Self::in_order_traverse(&node.left, arr);
            arr.push(node.val);
            Self::in_order_traverse(&node.right, arr);
        } else {
            return;
        }
    }
}

fn main() {
    println!("Hello, world!");
}

