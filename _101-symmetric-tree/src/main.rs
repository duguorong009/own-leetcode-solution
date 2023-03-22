use std::borrow::Borrow;
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

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(root) = root {
        let root = (*root).borrow();
        check_symmetric(&root.left, &root.right)
    } else {
        true
    }
}

fn check_symmetric(
    left: &Option<Rc<RefCell<TreeNode>>>,
    right: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    left == right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let node_left_3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node_left_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: node_left_3,
        })));
        let node_right_3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node_right_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: node_right_3,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node_left_2,
            right: node_right_2,
        })));
        assert_eq!(is_symmetric(root), true);
    }

    #[test]
    fn test_2() {
        let node_left_3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node_left_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: node_left_3,
        })));
        let node_right_3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node_right_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: node_right_3,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node_left_2,
            right: node_right_2,
        })));
        assert_eq!(is_symmetric(root), false);
    }
}
