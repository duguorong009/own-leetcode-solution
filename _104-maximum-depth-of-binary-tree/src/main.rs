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

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    node_max_depth(&root)
}

fn node_max_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = node {
        let node = node.borrow();
        return 1 + node_max_depth(&node.left).max(node_max_depth(&node.right));
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let node_7 = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        })));
        let node_15 = Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: None,
            right: None,
        })));
        let node_20 = Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: node_15,
            right: node_7,
        })));
        let node_9 = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node_9,
            right: node_20,
        })));
        assert_eq!(max_depth(root), 3);
    }
}
