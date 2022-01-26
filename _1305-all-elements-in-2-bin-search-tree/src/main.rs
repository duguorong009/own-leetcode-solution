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

fn main() {
    println!("Hello, world!");
}

fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut v1: Vec<i32> = vec![];
    let mut v2: Vec<i32> = vec![];
    inorder(&root1, &mut v1);
    inorder(&root2, &mut v2);
    let n = v1.len();
    let m = v2.len();
    let mut i = 0;
    let mut j = 0;
    while i < n || j <m { 
        if i == n {
            res.push(v2[j]);
            j += 1;
            continue;
        }
        if j == m {
            res.push(v1[i]);
            i += 1;
            continue;
        }
        if v1[i] < v2[j] {
            res.push(v1[i]);
            i += 1;
        } else {
            res.push(v2[j]);
            j += 1;
        }
    }
    res
}

fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    if let Some(node) = node {
        let node = node.borrow();
        let val = node.val;
        let left = &node.left;
        let right = &node.right;
        inorder(left, v);
        v.push(val);
        inorder(right, v);
    }
}