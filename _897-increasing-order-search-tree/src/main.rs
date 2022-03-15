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

fn increasing_bst(
    root: Option<Rc<RefCell<TreeNode>>>
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut arr: Vec<i32> = vec![];
    
    in_order_traversal(&root, &mut arr);

    let mut new_root: Option<Rc<RefCell<TreeNode>>> = None;
    for i in (0..arr.len()).rev() {
      new_root = Some(Rc::new(RefCell::new(TreeNode { val: arr[i], left: None, right: new_root })));
    }
    new_root
}

fn in_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
  if let Some(node) = root {
    let node = node.borrow();
    let left_tree = &node.left;
    let right_tree = &node.right;
    in_order_traversal(&left_tree, arr);
    arr.push(node.val);
    in_order_traversal(right_tree, arr);
  } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let left = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None})));
        let right = Some(Rc::new(RefCell::new(TreeNode { val: 7, left: None, right: None })));
        let root = Some(Rc::new(RefCell::new(TreeNode {val: 5, left: left, right: right})));
        assert_eq!(increasing_bst(root), None);
    }
}